#!/bin/bash
echo "Running tests..."

echo "[JavaScript]"
node JavaScript/primary.test.js || exit 1

echo "[Python]"
cd "pypy and python" && python3 -m unittest test_primary.py || exit 1
cd ..

echo "[Go]"
cd go && go test || exit 1
cd ..

echo "[Rust]"
cd rust && rustc --test main.rs && ./main || exit 1
cd ..

echo "[C++]"
cd cpp && g++ -DTEST main.cpp -o test_main && ./test_main || exit 1
rm cpp/test_main
cd ..

echo "[Java]"
cd java && javac Main.java MainTest.java && java -ea MainTest || exit 1
rm java/*.class
cd ..

echo "All tests passed successfully!"
