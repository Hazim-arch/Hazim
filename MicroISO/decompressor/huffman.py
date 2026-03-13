class DeHuffman:
    def __init__(self, file):
        print("[+] starting huffman compresstion")
        self.file = file
        self.MAP = {
        b' ': b'\x00', b';': b'\x01', b',': b'\x000', b'(': b'\x001', b')': b'\x010',
        b'{': b'\x011', b'}': b'\x0000', b'e': b'\x0111', b't': b'\x0010', b'n': b'\x0100',
        b'i': b'\x0011', b's': b'\x0110', b':': b'\x0101', b'.': b'\x00000', b'_': 
        b'\x00001', b'r': b'\x00010', b'a': b'\x00100', b'o': b'\x01000', b'l': b'\x00011', 
        b'\n': b'\x00110', b'>': b'\x01100', b'<': b'\x00101', b'=': b'\x01010', b'u': 
        b'\x00111', b'f': b'\x01110', b'\t': b'\x01111', b'm': b'\x000000', b'"': 
        b'\x000001', b'p': b'\x000010', b'[': b'\x000100', b']': b'\x001000', b'c': 
        b'\x010000', b'/': b'\x000011', b'd': b'\x000110', b"'": b'\x001100', b'&': 
        b'\x011000', b'*': b'\x000101', b'y': b'\x001010', b'!': b'\x010100', b'+': 
        b'\x000111', b'?': b'\x001110', b'|': b'\x011100', b'0': b'\x001111', b'1': 
        b'\x011110', b'2': b'\x0000000', b'v': b'\x0000001', b'T': b'\x000010', b'S': 
        b'\x0111000', b'R': b'\x0001000', b'O': b'\x0010000', b'P': b'\x0100000', b'M': 
        b'\x0000011', b'C': b'\x0000110', b'h': b'\x0001100', b'F': b'\x0000101',
        b'L': b'\x0001110', b'A': b'\x0010100', b'D': b'\x0101000', b'U': b'\x0001001', 
        b'I': b'\x0010010',b'E': b'\x0100100', b'V': b'\x0010001', b'B': b'\x0100010', 
        b'N': b'\x0000111', b'': b'\x0011100'
    }

        self.processed_chunk = bytearray()
        self.chunk_size = 64 * 1024

        while True:
            self.chunk = file.read(1024) 
            print("[+] set reading size to 1GB")
            if not self.chunk:
                break

    def compress(self, data):
        self.file.seek(0)
        while True:
            self.chunk = self.file.read(self.chunk_size)

            if not self.chunk:
                print("[+] decompretion comlete")
                break

            for i in range(len(self.chunk)):
                byte = self.chunk[i:i+1]
                self.processed_chunk.extend(self.MAP.get(byte, byte))
            
            print('[+] huffman decompression complete')
