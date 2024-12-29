from random import randrange
from math import factorial
# bijection between Z^2 and N
def bij(pos):
    x,y=pos
    r=abs(x)+abs(y)
    p=[y, r-x, 3*r+x, 2*r-y][(y<0)*2+(x<0)]
    return r*(r-1)*2+(r>0)+p

# Feistel cipher + linear congruential generator 
def encrypt(num, key):
    b=num.bit_length()
    bm=(b+1)//2
    mask=(1<<bm)-1
    l,r=num>>bm,num&mask
    k_block_len=20
    k_mask=(1<<k_block_len)-1
    while key>0:
        block=key&k_mask
        key>>=k_block_len
        block=block*2+1
        for _ in range(10): l,r=((l*block+1)&mask)^r, l

    return (l<<bm|r|(1<<bm*2))-1

def rand_uint(prob):
    res=0
    while randrange(prob)>0:
        res=(res<<10)|randrange(1<<10)
    return res

def rand_int(prob):
    return rand_uint(prob)*(randrange(2)*2-1)

def get_num(key, hxpos, wallno):
    return encrypt(bij(hxpos)*4+wallno, key)

def get_text(key, hxpos, wallno):
    n=get_num(key, hxpos, wallno)
    all_chars="abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.!?() "
    num_chars=len(all_chars)
    res=""
    while n>0:
        res+=all_chars[n%num_chars]
        n//=num_chars
    return res

def chunks(s, csize):
    l=len(s)
    return (s[i:i+csize]+" "*(max(0, i+csize-l)) for i in range(0, l, csize))

def show_text(key, hxpos, wallno):
    t=get_text(key, hxpos, wallno)
    width=100
    margin=5
    print("╔"+"═"*(margin*2+width)+"╗")
    for i in range(margin): print("║"+" "*(margin*2+width)+"║")
    for l in chunks(t, width):
        print("║"+" "*margin+l+" "*margin+"║")
    for i in range(margin): print("║"+" "*(margin*2+width)+"║")
    print("╚"+"═"*(margin*2+width)+"╝")

def add(a, b):
    return tuple(a[i]+b[i] for i in range(min(len(a), len(b))))

intro="""
You don't quite remember what happened, but as you get more time to recover
from the shock at the surreality of your surroundings, you start to remember 
how you got here: that damn pit in the ground. You had noticed it, but too late. 
As you try to remember the details of your fall, a distinct text appears in your mind. 
It read:
"There are infinite locks, but only one key. Fortunately you need only one to escape."
Suspiciously, it was written in the same font as the walls of text in front of you.
The walls are tall, and full of text. Seemingly gibberish text....

As you take a look around, you see that the room is octagonal in shape and actually has 
4 walls, the other 4 being doors, seemingly, into other rooms. As you look down, 
you see a lock in the center of the room, and immediately the text about the locks come
into your mind. Was it actually true? Who wrote that? Why? These
questions linger in your mind, but you feel your hunger growing and realize you need
to get out of here as soon as possible...
"""

help_msg="""
h -- show this message.
l/r -- turn left/right.
m -- move into the room behind the door in front of you.
s -- show what's before you.
k -- enter the key to escape.
q -- quit this game.
"""

win_msg="""
Aaaand you escaped that hellscape for good. What a journey it was! You take
one last look at the pit you fell into as you run back into your vehicle, 
while promising yourself to never come here ever again. But a part of you
wishes to go back there, to spend a little more time reading and pondering
about those strange, beautiful, texts....
"""

print(intro)
prob=5000
key=rand_uint(5*prob)
hxpos=(rand_int(prob), rand_int(prob))
facedir=randrange(8)
directions=[(0, -1), (-1, 0), (0, 1), (1, 0)]
validcmds="hlrmsqk"

while True:
    inp=input("> ").strip()
    if inp not in validcmds or inp == "h":
        print(help_msg)
    elif inp=="k":
        k=int(input("Enter key: "))
        if k==key:
            print(win_msg)
            break
        print("Wrong!")
    elif inp in "lr":
        facedir+=(inp=="r")*2-1
        facedir&=7
    elif inp == "m":
        if facedir&1==0:
            hxpos=add(hxpos, directions[facedir>>1])
        else: print("You must be facing a door to be able to move through it.")
            
    elif inp == "s":
        if facedir&1:
            print("The text in the wall before you reads:")
            show_text(key, hxpos, facedir>>1)
        else: print("A black, rusty door stands before you.")
    elif inp=="q": break
    else: print("wtf")
