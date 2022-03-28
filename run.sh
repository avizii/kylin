cd user

cargo build --release

sh ./application-build.sh

cd ..

sh ./build.sh

sh ./start-kylin.sh