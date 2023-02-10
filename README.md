# Contract-To-Interface

Simple CLI tool to create Interface from a Solidity smart contract. 

## Motivations

Sometimes when you write a big contract with many functoins it is very annoying to go through the functions one by one and copy them to a new file. 
This tool will parse any smart contract and will wirte the interface for you.

## Usage
1. Clone this repo:
```bash
git clone https://github.com/Kuly14/contract-to-interface.git
```

2. Run the program
```bash
cargo r -- ~/path/to/your/contract.sol 
```
By default it will create a new file with the same name as the contract but will add `I` in front of the name, so: Contract.sol -> IContract.sol
This is a standard practice.

If you also want to print the interface to the terminal add `-p`
```bash
cargo r -- ~/path/to/your/contract.sol -p
```

If you want to save the interface in a specific directory you can do so with `-d` or `--destination`:
```bash
cargo r -- ~/path/to/your/contract.sol --destination ~/path/to/your/destination
```

## Limitations

As of now this tool only works for a flattened smart contracts, and only if there is one smart contract in the file.

## Disclaimer
This is untested unaudited tool, do not use in production! Always cross reference if the interface has the right name and params, since if it varied from the smart contract the compiler will calculate wrong function selector and therefore any external call to that function will always fail.

## TODO

- [ ] Add flatten functionality.
- [ ] Modify the parsing so it will work if there are multiple contracts in the same file.
