import os
from Lzma import XZ
from huffman import Huffman
import sys

class Compresser:
    def __init__(self, file):
        self.file = file
        print("[+] picked up file {}", file)

        if not os.path.exists(self.file):
            print(f"[+] file '{self.file}' dos'nt exist")
            sys.exit()

        print("[+] opening in read byte mode")
        self.f = open(self.file, 'rb')

        self.Huffman = Huffman(self.f)
        self.XZ = XZ(self.f)
        
    def run(self):
        data = self.Huffman.compress()
        self.XZ.run(data)

if __name__ == '__main__':
    inp = input()
    app = Compresser(inp)
    app.run()