#!/bin/bash

USER="watcher"
GROUP="watcher"

# Create user for watcher and add it to video group
if ! id -u $USER > /dev/null 2>&1; then
    sudo groupadd $GROUP
    sudo useradd -m -g $GROUP $USER
    sudo usermod -aG video $USER
fi

# Build the executable and copy it to /usr/local/bin
cargo build --release
sudo cp ./target/release/watcher /usr/local/bin/watcher

# Set up directories
DIRECTORIES=(
    /var/log/watcher
    /var/lib/watcher
    /var/lib/watcher/pictures
)
for DIR in "${DIRECTORIES[@]}"; do
    sudo mkdir -p "$DIR";
    sudo chown -R "$USER:$USER" "$DIR";
done

# Create log files
sudo touch /var/log/watcher/pictures.log
sudo touch /var/log/watcher/pictures_error.log
sudo chown -R "${USER}:${USER}" /var/log/watcher;

# Create service file
sudo tee /etc/systemd/system/watcher.service > /dev/null <<EOL
[Unit]
Description=Service for watcher
After=network.target

[Service]
ExecStart=/usr/local/bin/watcher
WorkingDirectory=/var/lib/watcher
User=$USER
Group=$GROUP
Restart=always
Environment=DEVICE_NUMBER=0
Environment=INTERVAL=3600
Environment=LOG_DIR=/var/log/watcher
Environment=DATA_DIR=/var/lib/watcher/pictures

[Install]
WantedBy=multi-user.target
EOL

# Setup service
sudo systemctl daemon-reload
sudo systemctl enable watcher.service
sudo systemctl start watcher.service
