2.0.0
=====

* Update to reed-solomon-32 version 2.0.0. Significantly, that version pre-computes
  values that used to be computed at runtime. This allows us to simplify our
  API to primary just use free-functions that use those pre-computed tables as opposed
  to having to manually manage objects that contain runtime-computed values.
* Some users may wish to keep binary size to a minimum (eg, such as for WASM output).
  In order to minimize binary size, we create a lower-level interface that allows
  for a particular ECC size to be chosen at compile time. When using this interface,
  unneeded precomputed tables may be eliminated from the final binary at link time.
* Update to libzbase 2.0.0 - this doesn't impact the features of this library.
* Update the Error APIs to make it easier to differentiate between input related
  errors and usage related errors.
* Fix incorrect use statements that were preventing the no_std mode from actually
  working.
