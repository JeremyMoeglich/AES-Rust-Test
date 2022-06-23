pm2 stop ./build/index.js
pm2 start ./build/index.js --name "node-server"
cd ./Java-Huffman-Test
gradle bootRun