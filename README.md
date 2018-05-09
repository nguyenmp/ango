# ango

An educational experiment to develop a password manager for iOS, Android, and macOS.

# Unplanned

* Decentralized multi-factor authentication.  To share passwords between devices, a previous device must consent to adding the new device to the group.  All devices get notified and know who consented.
* Complete, blamable history.  Any changes made in the storage can be tracked to which device made that change.
* Full offline capabilities.  The only feature you lose by being offline is syncing new updates.  You have full access to all your passwords offline and can make edits that will get pushed when you come back.

# Research and Notes

## Crypto
http://latacora.singles/2018/04/03/cryptographic-right-answers.html
https://news.ycombinator.com/item?id=16748400
https://gist.github.com/TheZ3ro/fb521a3cde0c91fcb350

## libsodium

```bash
make clean
./configure --host x86_64-sun-solaris
make
make install DESTDIR=/Users/mark/Downloads/libsodium-1.0.16/result/
```

# Cross compiling

https://github.com/japaric/rust-cross#i-want-to-build-binaries-for-linux-mac-and-windows-how-do-i-cross-compile-from-linux-to-mac

The rough idea is that I don't cross compile between different platforms, except Android and iOS because they are designed that way with limited resources for compiling.  If I want windows, I'll probably either download the pre-build windows assets or build from source on a windows machine.  Same with mac.  I think this is okay given that I won't develop for it unless I use it.

# Static Linking

First complile agains the lib sodium headers: `gcc -c -I../libsodium-1.0.16/src/libsodium/include/ foo.c`

Then statically link to libsodium: `gcc -o foo foo.o ../libsodium-1.0.16/src/libsodium/.libs/libsodium.a`

Or dynamiucally linking: `gcc -o foo foo.o -lsodium`

All in one: `gcc -o foo foo.c ../libsodium-1.0.16/src/libsodium/.libs/libsodium.a`

I can probably borrow the above for rustc, I just need to figure out how to do this with the output of build.rs:

https://doc.rust-lang.org/cargo/reference/build-scripts.html
