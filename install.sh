# Removing Old Version of Peaches
rm -rf ~/.local/bin/peaches

# Get Latest Release from Github
wget https://github.com/KCaverly/peaches/releases/download/v0.2.0/peaches --no-check-certificate -P ~/.local/bin/
chmod +x ~/.local/bin/peaches
