# Group #3

## Members
Dawid Wasniowski (dawidw2)
Will Blum (wblum2)
David Augustynski(daugu)

##Project Introduction
Our project is to create a Rust application that allows users to encrypt and decrypt text files using multiple simple encryption algorithms. It’ll feature a frontend UI that will allow for user input, and display the encrypted and decrypted text. 

The goal of this project is to explore different encryption algorithms while gaining more experience with Rust, and file handling. We chose this project because it combines both practical programming with some technical aspects, while allowing us to explore data security and processing.

## Possible Challenges
File reading, learning new algorithms, learning how to package our code, figuring out how to reverse our encryptions to be able to decrypt our encryption as well, learning how to create a UI.

## Objectives
- Implement several encryption algorithms
- Build a file reading CLI that can use these algorithms
- Export our program as a Rust crate
- Create a simple frontend UI that uses our Rust crate if time permits

## Technical Overview
- CLI Interface with file input and output
- Encryption Struct with 5 different methods (encryption algorithms)
- Caesar Cipher: Shifts characters by any amount
- Vigenere Cipher: Uses Caesar Ciphers of different shift amounts based on the key
- Substitution Cipher: Replaces each letter with another letter based on a mapping
- XOR Cipher: Applies a bitwise XOR operation to each character in the string
- Rail Wise Cipher: Encrypts a message with a zig-zag pattern with a set height
- Package our encryption class as a crate
- Frontend UI (using HTML, CSS, JS)  with a dropdown for picking encryption algorithm, textbox for input, and read-only textbox for output. Encryption uses our rust crate.

## Checkpoint Goals
By checkpoint #1: Set up file reader and finish 2 of the encryption algorithms
By checkpoint #2: Finish the rest of the 3  encryption algorithms
