import os
from Lzma import DeXZ
from huffman import DeHuffman
import sys

class Decomporessor:
    def __init__(self, file):
        self.file = file
        
        if not os.path.exists(self.file):
            print(f"[+] file '{self.file}' dos'nt exist")
            sys.exit()

        self.f = open(self.file, 'rb')

        self.Huffman = DeHuffman(self.f)
        self.XZ = DeXZ(self.f)
        
    def run(self):
        data = self.XZ.run()
        self.Huffman.compress(data)
        with open("kernal.bin", 'wb') as self.fil:
            self.fil.write(data)

if __name__ == '__main__':
    imp = input('Enter miso file: ')
    decomporessor = Decomporessor(imp)
    decomporessor.run()