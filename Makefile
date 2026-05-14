build:
	scp -r src Cargo.lock Cargo.toml .cargo pi@raspberrypi.local:hal3000/
	ssh pi@raspberrypi.local "cd /home/pi/hal3000 && /home/pi/.cargo/bin/cargo build"

run:
	ssh pi@raspberrypi.local "sudo /home/pi/hal3000/target/debug/hal3000"

stop:
	ssh pi@raspberrypi.local "sudo killall hal3000"
