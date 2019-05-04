pacman -Sy --noconfirm docker
pacman -Sy --noconfirm docker-compose
systemctl enable docker
systemctl start docker
sudo usermod -aG docker vagrant
