
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
sudo ln -s `pwd`/wowthing_upload.service /etc/init.d/wowthing_upload
sudo service wowthing_upload enable
sudo service wowthing_upload start
```

## check logs

```bash
sudo journalctl -u wowthing_upload
```

## uninstall

```bash
sudo service wowthing_upload stop
sudo service wowthing_upload disable
sudo rm /etc/init.d/wowthing_upload
```
