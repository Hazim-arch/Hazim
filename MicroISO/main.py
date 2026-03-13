from decompressor.main import Decomporessor
import subprocess
import os

class MAIN:
    def __init__(self):
        print("Enter miso file")
        self.input = input("")
        self.decompressor = Decomporessor(self.input)

        print("[+] flashing ready select device")
        self.flash(input("enter image path"), input("enter device path"))

    def flash(self, image_path, drive_path):
        self.image_path = image_path
        self.drive_path = drive_path

        if os.geteuid() != 0:
            print("[!] Error: You must run this as sudo!")
            return
        
        print(f"!!! WARNING !!!")
        print(f"Target: {self.drive_path}")
        print(f"Source: {self.image_path}")
        self.confirm = input("This will WIPE the target drive. Type 'YES' to proceed: ")
        if self.confirm != "YES":    
            print("Aborted.")
            return

        try:
            print(f"[+] Opening {self.drive_path}...")
            with open(self.image_path, 'rb') as self.src:
                with open(self.drive_path, 'wb') as self.dest:
                    self.data = self.src.read()
                    self.dest.write(self.data)
                    self.dest.flush()
                    os.fsync(self.dest.fileno())
                        
                print("[+] Flash Complete! LunerOS is now on the drive.")
        except Exception as e:
            print(f"[!] Flash Failed: {e}")

if __name__ == "__main__":
    app = MAIN