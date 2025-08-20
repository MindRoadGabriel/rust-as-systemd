cargo build --release
cp ./target/release/rust-as-systemd /usr/bin/
cp ./rust-as-systemd.service /lib/systemd/system/.
systemctl daemon-reload
systemctl restart rust-as-systemd
sleep 4
journalctl -f -u rust-as-systemd | less
ps aux | grep rust | less
