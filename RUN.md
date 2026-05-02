# How to Run the Encryption CLI

1. Clone the repository:

   git clone https://github.com/Daw155/encrypt.git

2. Navigate into the project directory:

   cd encrypt

3. Run the project using Cargo:

   cargo run

   or

   cargo run -- -t "hello, world!" -c railfence -k 2

   -t is for the text you want to encrypt or decrypt
   -f is for the file you want to encrypt or decrypt
   -c is for the cipher you want to use
   -k is for the key you want to use
   -d is for decrypt (optional, if not included, it will encrypt)

4. Follow the prompts in the terminal to choose a cipher, input text or a file path, and a key to perform the encryption or decryption.
