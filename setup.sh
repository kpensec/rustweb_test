pacman -Sy --noconfirm docker
pacman -Sy --noconfirm docker-compose
pacman -Sy --noconfirm redis
systemctl enable docker
systemctl start docker
sudo usermod -aG docker vagrant
