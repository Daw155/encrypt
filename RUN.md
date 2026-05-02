# How to Run the Encryption CLI

1. Clone the repository:

   git clone https://github.com/Daw155/encrypt.git

2. Navigate into the project directory:

   cd encrypt

3. Run the project using Cargo:

   cargo run

   or

   cargo run -- -t "hello, world!" -c railfence -k 2

   * `-t`: The text you want to encrypt or decrypt
   * `-f`: The file you want to encrypt or decrypt (use instead of `-t`)
   * `-c`: The cipher you want to use (caesar, vigenere, columnar, xor, railfence)
   * `-k`: The key or shift amount you want to use
   * `-d`: Flag to decrypt (optional, if not included, it will encrypt)

4. Follow the prompts in the terminal to choose a cipher, input text or a file path, and a key to perform the encryption or decryption.
