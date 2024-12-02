
# wowthing_upload Service

Uploads WoWthing data to WowThing.
its a basic replacement for the WowThing app. This version works as a service.

## Note

even though this is a linux service, it runs using the windows python.exe and
therefore requires the windows file paths.
This is due to the `watchdog` library not triggering file events on linux when
monitoring windows drives.

## install

- add your api key to the wowthing_upload.service file
- add the paths to the wowthing files to the wowthing_upload.service file

```bash
sudo ln -s ./wowthing_upload.service /etc/systemd/system/wowthing_upload.service
sudo systemctl enable wowthing_upload
sudo systemctl start wowthing_upload
```

## check logs

```bash
sudo systemctl status wowthing_upload
```

## uninstall

```bash
sudo systemctl stop wowthing_upload
sudo systemctl disable wowthing_upload
sudo rm /etc/init.d/wowthing_upload
```
