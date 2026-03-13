import lzma

class XZ:
    def __init__(self, f):
        print("[+]starting XZ(lzma) compretion")
        self.f = f
        print("[+] compressor is initalised")
        self.chunk_size = 64 * 1024
        print(f"[+] chuck size is set to {self.chunk_size}")
        self.compressor = lzma.LZMACompressor(format=lzma.FORMAT_XZ)

    def run(self, data):
        print("[+] starting comprestion")

        self.compressed = self.compressor.compress(data)
        self.final_data = self.compressed + self.compressor.flush() 
        
        print(f"[+] MicroISO complete! Result saved to kernel.miso")

        with open("kernal.miso", "wb") as f_out:
            f_out.write(b'MISO') # Your LunerOS Header [cite: 1, 37]
            f_out.write(self.final_data) 

        
            