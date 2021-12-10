from urllib.request import urlopen, Request
import argparse
import json
from pathlib import Path


def transfer(from_file, to_file):
    while True:
        buf = from_file.read(16384)
        if not buf:
            return
        while buf:
            cnt = to_file.write(buf)
            buf = buf[cnt:]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("tag_name")
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    args.output.mkdir(exist_ok=True, parents=True)

    request = Request(
        url=f"https://api.github.com/repos/DaGenix/libhumancode/releases/tags/{args.tag_name}",
        headers={
            "Accept": "application / vnd.github.v3 + json",
        }
    )
    with urlopen(request, timeout=60) as response:
        if response.status != 200:
            raise Exception("Failed to get release")
        release_info = json.loads(response.read())

    for asset_info in release_info["assets"]:
        asset_name = asset_info["name"]
        asset_url = asset_info["browser_download_url"]
        with open(args.output / asset_name, "wb") as out_file:
            with urlopen(asset_url, timeout=60) as asset_response:
                if response.status != 200:
                    raise Exception("Failed to get file")
                transfer(asset_response, out_file)


if __name__ == "__main__":
    main()
