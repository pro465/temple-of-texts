from tot import *

def test_encryption(times):
    for _ in range(times):
        num=rand_uint(1000)
        key=rand_uint(1000)
        print(f"key: {key:x}\nnumber: {num:x}")
        enc=encrypt(num, key)
        dec=decrypt(enc, key)
        print(f"decrypted: {dec:x}\nencrypted: {enc:x}")
        assert dec==num
        dec=decrypt(num, key)
        enc=encrypt(dec, key)
        print(f"decrypted: {dec:x}\nencrypted: {enc:x}")
        assert enc==num
    print("encryption/decryption test successful")

def find_match_length(a, b):
    while a!=b:
        a>>=1
        b>>=1
    return a.bit_length()

def test_convergence(times):
    m=0
    for _ in range(times):
        num=rand_uint(1000)
        key=rand_uint(1000)
        print(f"key: {key:x}\nnumber: {num:x}")
        
        enc1=encrypt(num, key)
        enc2=encrypt(num+1, key)
        m+=find_match_length(enc1, enc2)/times
    print(f"average match length: {m}")


test_encryption(1000)
test_convergence(1000)
