use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(mymodule, HumancodeException, PyException);
create_exception!(mymodule, TooManyErrors, HumancodeException);
create_exception!(mymodule, UsageException, HumancodeException);

#[pyfunction]
#[pyo3(name = "encode_chunk")]
fn py_encode_chunk(py: Python, data: &[u8], bits: u32, ecc: u32) -> PyResult<String> {
    py.allow_threads(|| {
        let bits = if bits <= 255 {
            bits as u8
        } else {
            return Err(UsageException::new_err("bits value is too large"));
        };
        let ecc = if ecc <= 255 {
            ecc as u8
        } else {
            return Err(UsageException::new_err("ecc value is too large"));
        };
        match libhumancode::encode_chunk(data, ecc, bits) {
            Ok(result) => Ok(result.pretty().as_str().to_string()),
            Err(libhumancode::UsageError(cause)) => Err(UsageException::new_err(cause.to_string())),
        }
    })
}

#[pyclass]
struct DecodeOutput {
    decode_output: libhumancode::DecodeOutput,
}

#[pymethods]
impl DecodeOutput {
    fn data(&self) -> PyResult<Vec<u8>> {
        Ok(self.decode_output.data().to_vec())
    }

    fn had_errors(&self) -> PyResult<bool> {
        Ok(self.decode_output.had_errors())
    }

    fn corrected_chunk(&self) -> PyResult<String> {
        Ok(self
            .decode_output
            .corrected_chunk()
            .pretty()
            .as_str()
            .to_string())
    }
}

#[pyfunction]
#[pyo3(name = "decode_chunk")]
fn py_decode_chunk(py: Python, data: &str, bits: u32, ecc: u32) -> PyResult<DecodeOutput> {
    py.allow_threads(|| {
        let bits = if bits <= 255 {
            bits as u8
        } else {
            return Err(UsageException::new_err("bits value is too large"));
        };
        let ecc = if ecc <= 255 {
            ecc as u8
        } else {
            return Err(UsageException::new_err("ecc value is too large"));
        };
        match libhumancode::decode_chunk(data, ecc, bits) {
            Ok(decode_output) => Ok(DecodeOutput { decode_output }),
            Err(libhumancode::DecodeError::TooManyErrors) => Err(TooManyErrors::new_err(
                "The input message contained too many errors",
            )),
            Err(libhumancode::DecodeError::UsageError(cause)) => {
                Err(UsageException::new_err(cause.to_string()))
            }
        }
    })
}

#[pymodule]
#[pyo3(name = "libhumancode")]
fn module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("HumancodeException", py.get_type::<HumancodeException>())?;
    m.add("TooManyErrors", py.get_type::<TooManyErrors>())?;
    m.add("UsageException", py.get_type::<UsageException>())?;
    m.add_class::<DecodeOutput>()?;
    m.add_function(wrap_pyfunction!(py_encode_chunk, m)?)?;
    m.add_function(wrap_pyfunction!(py_decode_chunk, m)?)?;
    Ok(())
}
