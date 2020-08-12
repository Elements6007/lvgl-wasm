# Build and install Wayland EGL app

# TODO: Install GLES2 library
# sudo apt install libgles2-mesa-dev

# Kill the app if it's already running
pkill lvgl

# Stop the script on error, echo all commands
set -e -x

# Build app
make

# Make system folders writeable
sudo mount -o remount,rw /

# Copy app to File Manager folder
cd wayland
sudo cp lvgl /usr/share/click/preinstalled/.click/users/@all/com.ubuntu.filemanager

# Copy run script to File Manager folder
# TODO: Check that run.sh contains "./lvgl"
sudo cp run.sh /usr/share/click/preinstalled/.click/users/@all/com.ubuntu.filemanager

# Set ownership on the app and the run script
sudo chown clickpkg:clickpkg /usr/share/click/preinstalled/.click/users/@all/com.ubuntu.filemanager/lvgl
sudo chown clickpkg:clickpkg /usr/share/click/preinstalled/.click/users/@all/com.ubuntu.filemanager/run.sh
ls -l /usr/share/click/preinstalled/.click/users/@all/com.ubuntu.filemanager/lvgl

# Start the File Manager
echo "*** Tap on File Manager icon on PinePhone"

# Monitor the log file
echo >/home/phablet/.cache/upstart/application-click-com.ubuntu.filemanager_filemanager_0.7.5.log
tail -f /home/phablet/.cache/upstart/application-click-com.ubuntu.filemanager_filemanager_0.7.5.log

# Press Ctrl-C to stop. To kill the app:
# pkill lvgl
