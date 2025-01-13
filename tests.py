from tot import *

def test_encryption(times, v=False):
    print("=== encryption/decryption test ===")
    for i in range(times):
        num=rand_uint(500)
        key=rand_uint(500)
        if v: print(f"key: {key:x}\nnumber: {num:x}")
        enc=encrypt(num, key)
        dec=encrypt(enc, key, t=True)
        if v: print(f"decrypted: {dec:x}\nencrypted: {enc:x}")
        assert dec==num
        dec=encrypt(num, key)
        enc=encrypt(dec, key, t=True)
        if v: print(f"decrypted: {dec:x}\nencrypted: {enc:x}")
        if i%100==0 and not v: print(".", end="", flush=True)
        assert enc==num
    print("\nencryption/decryption test successful")

def find_match_length(a, b):
    return max(a,b).bit_length()-(a^b).bit_length()

def test_divtopbits(times, thres):
    m=0
    print("=== MSD divergence test ===")

    for i in range(times):
        num=rand_uint(500)
        key=rand_uint(500)
        
        enc1=encrypt(num, key)
        enc2=encrypt(num+1, key)
        m+=find_match_length(enc1, enc2)/times
        if i%100==0: print(".", end="", flush=True)
    print(f"\naverage match length from MSD: {m}")
    print("ideal is 1")
    assert abs(m-1.)<thres
    print("value within threshold; MSD divergence test passed")

def count_nonmatches(a, b):
    return (a^b).bit_count()

def test_avalanche(times, thres):
    m=0
    print("=== avalanche test ===")
    for i in range(times):
        num=rand_uint(500)
        key=rand_uint(500)
        
        enc1=encrypt(num, key)
        enc2=encrypt(num+1, key)
        m+=count_nonmatches(enc1, enc2)/(enc2.bit_length()*times)
        if i%100==0: print(".", end="", flush=True)
    print(f"\naverage fraction of nonmatching bits: {m}")
    print("ideal is 0.5")
    assert abs(m-.5)<thres
    print("value within threshold; avalanche test passed")


test_encryption(1000)
test_divtopbits(5000, 0.1)
test_avalanche(5000, 0.01)
