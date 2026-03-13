class Huffman:
    def __init__(self, file):
        print("[+] starting huffman compresstion")
        self.file = file
        self.MAP = {
            b'\x00': b' ', b'\x01': b';',b'\x000': b',', b'\x001': b'(', b'\x010': b')',
            b'\x011': b'{', b'\x0000': b'}', b'\x0001': b'e', b'\x0010': b't',
            b'\x0100': b'n', b'\x0011': b'i', b'\x0110': b's', b'\x0101': b':', 
            b'\x0111': b'e', b'\x00000': b'.', b'\x00001': b'_', b'\x00010': b'r', 
            b'\x00100': b'a', b'\x01000': b'o', b'\x00011': b'l', b'\x00110': b'\n',
            b'\x01100': b'>', b'\x00101': b'<', b'\x01010': b'=', b'\x00111': b'u', 
            b'\x01110': b'f', b'\x01111': b'\t', b'\x000000': b'm', b'\x000001': b'"', 
            b'\x000010': b'p', b'\x000100': b'[', b'\x001000': b']', b'\x010000': b'c', 
            b'\x000011': b'/', b'\x000110': b'd', b'\x001100': b"'", b'\x011000': b"&", 
            b'\x000101': b'*', b'\x001010': b'y', b'\x010100': b'!', b'\x000111': b'+', 
            b'\x001110': b'?', b'\x011100': b'|', b'\x001111': b'0', b'\x011110': b'1', 
            b'\x0000000': b'2', b'\x0000001': b'v', b'\x000010': b'T', b'\x000100': b'S',
            b'\x0001000': b'R', b'\x0010000': b'O', b'\x0100000': b'P', b'\x0000011': b'M', 
            b'\x0000110': b'C', b'\x0001100': b'h', b'\x0000101': b'F', b'\x0001010': b'L', 
            b'\x0010100': b'A', b'\x0101000': b'D', b'\x0001001': b'U', b'\x0010010': b'I', 
            b'\x0100100': b'E', b'\x0010001': b'V', b'\x0100010': b'B', b'\x0000111': b'N', 
            b'\x0001110': b'L', b'\x0011100': b'', b'\x0111000': b'S'
        }

        self.processed_chunk = bytearray()
        self.chunk_size = 64 * 1024
        print("[+] set reading size to 1GB")
        
        while True:
            self.chunk = file.read(1024) 
            print("[+] reading")
            if not self.chunk:
                break

    def compress(self):
        print("[+] starting write")
        self.file.seek(0)
        while True:
            self.chunk = self.file.read(self.chunk_size)
            if not self.chunk:
                print("[+] compretion comlete")
                break

            for i in range(len(self.chunk)):
                byte = self.chunk[i:i+1]
                self.processed_chunk.extend(self.MAP.get(byte, byte))
            
            print('[+] huffman compression complete')
        return self.processed_chunk
