import logging
import os
import time

# IMPORT THIRD PARTY LIBRARIES
import tomllib
import requests
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler, FileSystemEvent


HOST = "https://wowthing.org"


LAST_CHANGE: dict[str, float] = {}
# dictionary to store the last change time for each file


# setup logging
LOGGER = logging.getLogger(__name__)
formatter = logging.Formatter("[%(asctime)s][%(levelname)s] %(message)s")
LOGGER.setLevel(logging.DEBUG)

# add console handler
handler = logging.StreamHandler()
handler.setFormatter(formatter)
LOGGER.addHandler(handler)

# add file handler
file_handler = logging.FileHandler("D:/dev/wowthing_upload/wowthing_upload.log")
file_handler.setFormatter(formatter)
LOGGER.addHandler(file_handler)


# load config
dir = os.path.dirname(os.path.abspath(__file__))
with open(f"{dir}/config.toml", "rb") as f:
    config = tomllib.load(f)
API_KEY = config["api_key"]
FILES = config["files"]


def upload_file(filepath: str) -> None:
    """
    Uploads the specified file's contents to the server.

    Args:
        filepath (str): The path of the file to upload.
    """
    LOGGER.info(f"Uploading file: {filepath}...")

    with open(filepath, "r", encoding="utf-8") as f:
        lua_content = f.read()

    data = {
        "ApiKey": API_KEY,
        "LuaFile": lua_content,
    }

    url = f"{HOST}/api/upload/"
    req = requests.post(url, json=data)
    req.raise_for_status()
    LOGGER.info(f"Upload complete: {filepath}")


class Handler(FileSystemEventHandler):

    def on_modified(self, event: FileSystemEvent) -> None:
        """
        Called when a file in the monitored directory is modified.

        Args:
            event: The event that triggered this method.
        """
        # print("on_modified", event.src_path)

        # wait for files to be written before checking for changes
        time.sleep(0.2)

        path = str(event.src_path)
        dirname = os.path.dirname(path)
        filepath = os.path.join(dirname, "WoWthing_Collector.lua")

        new_mtime = os.path.getmtime(filepath)
        old_mtime = LAST_CHANGE.get(filepath, 0)

        if new_mtime > old_mtime:
            LAST_CHANGE[filepath] = new_mtime

            LOGGER.info(f"File modified: {filepath}")

            try:
                upload_file(filepath)
            except Exception as e:
                LOGGER.error(f"Error uploading file: {filepath}")
                LOGGER.error(e)


def main():

    # info
    LOGGER.info("Starting WowThing Upload Service")
    for path in FILES:
        LOGGER.info(f"Monitoring file: {path}")

    handler = Handler()
    observer = Observer()
    for path in FILES:
        observer.schedule(handler, path=os.path.dirname(path))

    observer.start()

    try:
        while observer.is_alive():
            observer.join(1)
    finally:
        observer.stop()
        observer.join()

    LOGGER.info("WowThing Upload Service stopped")


if __name__ == "__main__":
    main()
