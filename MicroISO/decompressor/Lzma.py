import lzma

class DeXZ:
    def __init__(self, f):
        self.f = f
        self.decompressor = lzma.LZMADecompressor(format=lzma.FORMAT_XZ)

    def run(self):
        self.f.seek(0)
        header = self.f.read(4)

        if header != b'MISO':
            print("[!] Critical Failure: File is not a MISO image!")
            return None

        compressed_payload = self.f.read()
        
        if not compressed_payload:
            print("[!] Error: No data found after header!")
            return None

        print("[+] Decompressing LZMA layer...")
        try:
            decompressed_data = self.decompressor.decompress(compressed_payload)
            print(f"[+] Restored {len(decompressed_data)} bytes from XZ.")
            return decompressed_data 
        
        except Exception as e:
            print(f"[!] XZ Crash: {e}")
            return None