// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// import the verifier that the program created
import "./verifier.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

/// @title YourToken Contract
/// @notice This contract represents a unique ERC20 token with additional verification logic.
contract YourToken is ERC20 {
    Verifier public verifier;
    mapping(address => bool) public claimed;

    constructor() ERC20("YourToken", "YTK") {
        verifier = new Verifier();
    }

    /*
    * @notice Mints new tokens after verifying a provided proof.
    * @param input The inputs array from `addresses-proof.json`
    * @param proof The proof array from `addresses-proof.json`
    * @return A boolean value indicating whether the function executed successfully. Reverts otherwise.
    */
    function mint(uint256[3] memory input, Verifier.Proof memory proof) public returns (bool) {
        string memory senderString = Strings.toString(uint256(uint160(msg.sender)));
        uint256 midpoint = bytes(senderString).length / 2;

        string memory aString = substring(senderString, 0, midpoint);
        string memory bString = substring(senderString, midpoint, bytes(senderString).length);

        // Convert input values to decimal strings
        string memory input0String = hexToDecimalString(input[0]);
        string memory input1String = hexToDecimalString(input[1]);

        // Add leading zero if necessary
        input0String = addLeadingZeroIfNeeded(input0String, bytes(aString).length);
        input1String = addLeadingZeroIfNeeded(input1String, bytes(bString).length);

        // Ensure the sender matches the a and b inputs of the proof
        require(
            keccak256(abi.encodePacked(aString)) == keccak256(abi.encodePacked(input0String)),
            "Not your proof or invalid input"
        );
        require(
            keccak256(abi.encodePacked(bString)) == keccak256(abi.encodePacked(input1String)),
            "Not your proof or invalid input"
        );

        // Ensure the tokens haven't been claimed yet
        require(!claimed[msg.sender], "Tokens already claimed");

        // Verify the proof
        require(verifier.verifyTx(proof, input), "Invalid proof");

        // Mark as claimed and mint the tokens
        claimed[msg.sender] = true;
        _mint(msg.sender, 10 * 10 ** decimals());
        return true;
    }

    /*
    * @notice Extracts a substring from a given string.
    * @param str The original string.
    * @param startIndex The starting index from where to extract the substring.
    * @param endIndex The ending index to where extract the substring.
    * @return The extracted substring.
    */
    function substring(string memory str, uint256 startIndex, uint256 endIndex) internal pure returns (string memory) {
        bytes memory strBytes = bytes(str);
        bytes memory result = new bytes(endIndex-startIndex);
        for (uint256 i = startIndex; i < endIndex; i++) {
            result[i - startIndex] = strBytes[i];
        }
        return string(result);
    }

    /*
    * @notice Converts a uint value to its decimal string representation.
    * @param value The unsigned integer to convert.
    * @return The decimal string representation of the given unsigned integer.
    */
    function hexToDecimalString(uint256 value) internal pure returns (string memory) {
        return Strings.toString(value);
    }

    /*
    * @notice Adds a leading zero to a string if needed to reach a target length.
    * @param str The original string.
    * @param targetLength The target length for the resulting string.
    * @return The string with a leading zero added if needed.
    */
    function addLeadingZeroIfNeeded(string memory str, uint256 targetLength) internal pure returns (string memory) {
        uint256 strLength = bytes(str).length;
        if (strLength >= targetLength) {
            return str;
        }
        return string(abi.encodePacked("0", str));
    }
}
