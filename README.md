**1. DEPLOYING TO A TESTNET:**
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BasicMath {
    function adder(uint _a, uint _b) external pure returns (uint sum, bool error) {
        unchecked {
            uint result = _a + _b;
            if (result >= _a && result >= _b) {
                return (result, false);
            } else {
                return (0, true);
            }
        }
    }

    function subtractor(uint _a, uint _b) external pure returns (uint difference, bool error) {
        unchecked {
            if (_b <= _a) {
                return (_a - _b, false);
            } else {
                return (0, true);
            }
        }
    }
}
```


**2. CONTROL STRUCTURES:**
```solidity
// SPDX-License-Identifier: MIT

pragma solidity 0.8.17;

contract ControlStructures {
    error AfterHours(uint256 time);
    error AtLunch();

    function fizzBuzz(uint256 _number) public pure returns (string memory response) {
        if (_number % 3 == 0 && _number % 5 == 0) {
            return "FizzBuzz";
        } else if (_number % 3 == 0) {
            return "Fizz";
        } else if (_number % 5 == 0) {
            return "Buzz";
        } else {
            return "Splat";
        }
    }

    function doNotDisturb(uint256 _time) public pure returns (string memory result) {
        assert(_time < 2400);

        if (_time > 2200 || _time < 800) {
            revert AfterHours(_time);
        } else if (_time >= 1200 && _time <= 1299) {
            revert("At lunch!");
        } else if (_time >= 800 && _time <= 1199) {
            return "Morning!";
        } else if (_time >= 1300 && _time <= 1799) {
            return "Afternoon!";
        } else if (_time >= 1800 && _time <= 2200) {
            return "Evening!";
        }
    }
}
```

**3. STORAGE:**
```solidity
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.17;

contract EmployeeStorage {
    uint16 private shares;
    uint32 private salary;
    uint256 public idNumber;
    string public name;

    constructor(uint16 _shares, string memory _name, uint32 _salary, uint _idNumber) {
        shares = _shares;
        name = _name;
        salary = _salary;
        idNumber = _idNumber;
    }

    function viewShares() public view returns (uint16) {
        return shares;
    }
    
    function viewSalary() public view returns (uint32) {
        return salary;
    }

    error TooManyShares(uint16 _shares);
    function grantShares(uint16 _newShares) public {
        if (_newShares > 5000) {
            revert("Too many shares");
        } else if (shares + _newShares > 5000) {
            revert TooManyShares(shares + _newShares);
        }
        shares += _newShares;
    }

    /**
    * Do not modify this function.  It is used to enable the unit test for this pin
    * to check whether or not you have configured your storage variables to make
    * use of packing.
    *
    * If you wish to cheat, simply modify this function to always return `0`
    * I'm not your boss ¯\_(?)_/¯
    *
    * Fair warning though, if you do cheat, it will be on the blockchain having been
    * deployed by you wallet....FOREVER!
    */
    function checkForPacking(uint _slot) public view returns (uint r) {
        assembly {
            r := sload (_slot)
        }
    }

    /**
    * Warning: Anyone can use this function at any time!
    */
    function debugResetShares() public {
        shares = 1000;
    }
}
```

**4. ARRAYS:**
```solidity
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.17;

contract ArraysExercise {
    uint[] numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    uint[] timestamps;
    address[] senders;

    uint256 constant Y2K = 946702800; 

    function getNumbers() external view returns (uint[] memory) {
        uint[] memory results = new uint[](numbers.length);

        for(uint i=0; i<numbers.length; i++) {
            results[i] = numbers[i];
        }

        return results;
    }

    function resetNumbers() public {
        numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    }

    function appendToNumbers(uint[] calldata _toAppend) public {
        uint _counter = _toAppend.length;
        for (uint i; i < _counter; i++) {
            numbers.push(_toAppend[i]);
        }
    }

    function saveTimestamp(uint _unixTimestamp) public {
        timestamps.push(_unixTimestamp);
        senders.push(msg.sender);
    }

    function afterY2K() public view returns (uint256[] memory, address[] memory) {

      uint256 counter = 0;

      for (uint i = 0; i < timestamps.length; i++) {
           if (timestamps[i] > Y2K) {
               counter++;
           }
      }

        uint256[] memory timestampsAfterY2K = new uint256[](counter);
       address[] memory sendersAfterY2K = new address[](counter);

        uint256 index = 0;

        for (uint i = 0; i < timestamps.length; i++) {
            if (timestamps[i] > Y2K) {
                timestampsAfterY2K[index] = timestamps[i];
                sendersAfterY2K[index] = senders[i];    
                index++;
            }
       }

        return (timestampsAfterY2K, sendersAfterY2K);  
    }

    function resetSenders() public {
        delete senders;
    }

    function resetTimestamps() public {
        delete timestamps;
    }
}
```

**5. MAPPINGS:**
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

/**
 * @title FavoriteRecords
 * @dev Contract to manage a list of approved music records and allow users to add them to their favorites
 */
contract FavoriteRecords {

    mapping(string => bool) private approvedRecords;
    string[] private approvedRecordsIndex;

    mapping(address => mapping(string => bool)) public userFavorites;
    mapping(address => string[]) private userFavoritesIndex;

    error NotApproved(string albumName);

    /**
     * @dev Constructor that initializes the approved records list
     */
    constructor() {
        approvedRecordsIndex = ["Thriller","Back in Black","The Bodyguard","The Dark Side of the Moon","Their Greatest Hits (1971-1975)","Hotel California","Come On Over","Rumours","Saturday Night Fever"];
        for (uint i = 0; i < approvedRecordsIndex.length; i++) 
        {
            approvedRecords[approvedRecordsIndex[i]] = true;
        }
    }

    /**
     * @dev Returns the list of approved records
     * @return An array of approved record names
     */
    function getApprovedRecords() public view returns (string[] memory) {
        return approvedRecordsIndex;
    }

    /**
     * @dev Adds an approved record to the user's favorites
     * @param _albumName The name of the album to be added
     */
    function addRecord(string memory _albumName) public {
        if (!approvedRecords[_albumName]) {
            revert NotApproved({albumName: _albumName});
        }
        if (!userFavorites[msg.sender][_albumName]) {
            userFavorites[msg.sender][_albumName] = true;
            userFavoritesIndex[msg.sender].push(_albumName);
        }
    }

    /**
     * @dev Returns the list of a user's favorite records
     * @param _address The address of the user
     * @return An array of user's favorite record names
     */
    function getUserFavorites(address _address) public view returns (string[] memory) {
        return userFavoritesIndex[_address];
    }

    /**
     * @dev Resets the caller's list of favorite records
     */
    function resetUserFavorites() public {
        for (uint i = 0; i < userFavoritesIndex[msg.sender].length; i++) {
            delete userFavorites[msg.sender][userFavoritesIndex[msg.sender][i]];
        }
        delete userFavoritesIndex[msg.sender];
    }
}
```

**6. STRUCTS:**
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

/**
 * @title GarageManager
 * @dev Contract to manage a garage of cars for each user
 */
contract GarageManager {
    mapping(address => Car[]) private garages;

    struct Car {
        string make;
        string model;
        string color;
        uint numberOfDoors;
    }

    error BadCarIndex(uint256 index);

    /**
     * @dev Adds a new car to the caller's garage
     * @param _make The make of the car
     * @param _model The model of the car
     * @param _color The color of the car
     * @param _numberOfDoors The number of doors of the car
     */
    function addCar(string memory _make, string memory _model, string memory _color, uint _numberOfDoors) external {
        garages[msg.sender].push(Car(_make, _model, _color, _numberOfDoors));
    }

    /**
     * @dev Retrieves the caller's array of cars
     * @return An array of `Car` structs
     */
    function getMyCars() external view returns (Car[] memory) {
        return garages[msg.sender];
    }

    /**
     * @dev Retrieves a specific user's array of cars
     * @param _user The address of the user
     * @return An array of `Car` structs
     */
    function getUserCars(address _user) external view returns (Car[] memory) {
        return garages[_user];
    }

    /**
     * @dev Updates a specific car in the caller's garage
     * @param _index The index of the car in the garage array
     * @param _make The new make of the car
     * @param _model The new model of the car
     * @param _color The new color of the car
     * @param _numberOfDoors The new number of doors of the car
     */
    function updateCar(uint256 _index, string memory _make, string memory _model, string memory _color, uint _numberOfDoors) external {
        if (_index >= garages[msg.sender].length) {
            revert BadCarIndex({index: _index});
        }
        garages[msg.sender][_index] = Car(_make, _model, _color, _numberOfDoors);
    }

    /**
     * @dev Deletes all cars in the caller's garage
     */
    function resetMyGarage() external {
        delete garages[msg.sender];
    }
}
```

**7. INHERITANCE:**
```solidity
// SPDX-License-Identifier: MIT

pragma solidity 0.8.17;


abstract contract Employee
{
    uint public idNumber;
    uint public managerId;

    constructor(uint _idNumber, uint _managerId)
    {
        idNumber = _idNumber;
        managerId = _managerId;
    }

    function getAnnualCost() public virtual returns (uint);
}

contract Salaried is Employee
{
    uint public annualSalary;

    constructor(uint _idNumber, uint _managerId, uint _annualSalary)
        Employee(_idNumber, _managerId)
    {
        annualSalary = _annualSalary;
    }

    function getAnnualCost() public override view returns (uint)
    {
        return annualSalary;
    }
}

contract Hourly is Employee
{
    uint public hourlyRate;

    constructor(uint _idNumber, uint _managerId, uint _hourlyRate) Employee(_idNumber, _managerId)
    {
        hourlyRate = _hourlyRate;
    }

    function getAnnualCost() public override view returns (uint)
    {
        return hourlyRate * 2080;
    }
}

contract Manager
{
    uint[] public employeeIds;

    function addReport(uint _reportId) public 
    {
        employeeIds.push(_reportId);
    }

    function resetReports() public 
    {
        delete employeeIds;
    }
}

contract Salesperson is Hourly 
{
    constructor(uint _idNumber, uint _managerId, uint _hourlyRate) 
        Hourly(_idNumber, _managerId, _hourlyRate) {}
}


contract EngineeringManager is Salaried, Manager
{
    constructor(uint _idNumber, uint _managerId, uint _annualSalary) 
        Salaried(_idNumber, _managerId, _annualSalary) {}
}

contract InheritanceSubmission {
    address public salesPerson;
    address public engineeringManager;

    constructor(address _salesPerson, address _engineeringManager) {
        salesPerson = _salesPerson;
        engineeringManager = _engineeringManager;
    }
}
```

**8. IMPORTS:**

**SillyStringUtils.sol:**
```solidity
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.17;

library SillyStringUtils {

    struct Haiku {
        string line1;
        string line2;
        string line3;
    }

    function shruggie(string memory _input) internal pure returns (string memory) {
        return string.concat(_input, unicode" ??");
    }
}
```
**==========================================**

**Imports.sol:**
```solidity
// SPDX-License-Identifier: MIT

import "./SillyStringUtils.sol";

pragma solidity 0.8.17;

contract ImportsExercise {
    using SillyStringUtils for string;

    SillyStringUtils.Haiku public haiku;

    function saveHaiku(string memory _line1, string memory _line2, string memory _line3) public {
        haiku.line1 = _line1;
        haiku.line2 = _line2;
        haiku.line3 = _line3;
    }

    function getHaiku() public view returns (SillyStringUtils.Haiku memory) {
        return haiku;
    }

    function shruggieHaiku() public view returns (SillyStringUtils.Haiku memory) {
        SillyStringUtils.Haiku memory newHaiku = haiku;
        newHaiku.line3 = newHaiku.line3.shruggie();
        return newHaiku;
    }
}
```

**9. ERRORS:**
```solidity
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.17;

contract ErrorTriageExercise {
    /**
     * Finds the difference between each uint with it's neighbor (a to b, b to c, etc.)
     * and returns a uint array with the absolute integer difference of each pairing.
     */
    function diffWithNeighbor(
        uint _a,
        uint _b,
        uint _c,
        uint _d
    ) public pure returns (uint[] memory) {
        uint[] memory results = new uint[](3);

        results[0] = _a > _b ? _a - _b : _b - _a;
        results[1] = _b > _c ? _b - _c : _c - _b;
        results[2] = _c > _d ? _c - _d : _d - _c;

        return results;
    }

    /**
     * Changes the _base by the value of _modifier.  Base is always >= 1000.  Modifiers can be
     * between positive and negative 100;
     */
    function applyModifier(
        uint _base,
        int _modifier
    ) public pure returns (uint returnValue) {
        if(_modifier > 0) {
            return _base + uint(_modifier);
        }
        return _base - uint(-_modifier);
    }

    /**
     * Pop the last element from the supplied array, and return the popped
     * value (unlike the built-in function)
     */
    uint[] arr;

    function popWithReturn() public returns (uint returnNum) {
        if(arr.length > 0) {
            uint result = arr[arr.length - 1];
            arr.pop();
            return result;
        }
    }

    // The utility functions below are working as expected
    function addToArr(uint _num) public {
        arr.push(_num);
    }

    function getArr() public view returns (uint[] memory) {
        return arr;
    }

    function resetArr() public {
        delete arr;
    }
}
```

**10. THE "NEW" KEYWORD:**

**Change string private salt = "value" to any yours in both contracts.**

**AddressBook.sol**
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

import "@openzeppelin/contracts/access/Ownable.sol";

contract AddressBook is Ownable(msg.sender) {
    string private salt = "value"; 
    struct Contact {
        uint id;
        string firstName;
        string lastName;
        uint[] phoneNumbers;
    }

    Contact[] private contacts;
    mapping(uint => uint) private idToIndex;
    uint private nextId = 1;

    error ContactNotFound(uint id);

    function addContact(string calldata firstName, string calldata lastName, uint[] calldata phoneNumbers) external onlyOwner {
        contacts.push(Contact(nextId, firstName, lastName, phoneNumbers));
        idToIndex[nextId] = contacts.length - 1;
        nextId++;
    }

    function deleteContact(uint id) external onlyOwner {
        uint index = idToIndex[id];
        if (index >= contacts.length || contacts[index].id != id) revert ContactNotFound(id);

        contacts[index] = contacts[contacts.length - 1];
        idToIndex[contacts[index].id] = index;
        contacts.pop();
        delete idToIndex[id];
    }

    function getContact(uint id) external view returns (Contact memory) {
        uint index = idToIndex[id];
        if (index >= contacts.length || contacts[index].id != id) revert ContactNotFound(id);
        return contacts[index];
    }

    function getAllContacts() external view returns (Contact[] memory) {
        return contacts;
    }
}
```
**=======================================================**

**OTHER CONTRACT:**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

import "./AddressBook.sol";

contract AddressBookFactory {
    string private salt = "value";
    function deploy() external returns (AddressBook) {
        AddressBook newAddressBook = new AddressBook();
        newAddressBook.transferOwnership(msg.sender);
        return newAddressBook;
    }
}

```

**11 MINIMAL TOKENS:**

**Change string private salt = "value" to any yours.** 
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract UnburnableToken {
    string private salt = "value"; 
    mapping(address => uint256) public balances;
    uint256 public totalSupply;
    uint256 public totalClaimed;
    mapping(address => bool) private claimed;

    // Custom errors
    error TokensClaimed();
    error AllTokensClaimed();
    error UnsafeTransfer(address _to);

    constructor() {
        totalSupply = 100000000; // Set the total supply of tokens
    }

    // Public function to claim tokens
    function claim() public {
        if (totalClaimed >= totalSupply) revert AllTokensClaimed(); // Check if all tokens have been claimed
        if (claimed[msg.sender]) revert TokensClaimed(); // Check if the caller has already claimed tokens

        // Update balances and claimed status
        balances[msg.sender] += 1000;
        totalClaimed += 1000;
        claimed[msg.sender] = true;
    }

    // Public function for safe token transfer
    function safeTransfer(address _to, uint256 _amount) public {
        // Check for unsafe transfer conditions, including if the target address has a non-zero ether balance
        if (_to == address(0) || _to.balance == 0) revert UnsafeTransfer(_to);

        // Ensure the sender has enough balance to transfer
        require(balances[msg.sender] >= _amount, "Insufficient balance");

        // Perform the transfer
        balances[msg.sender] -= _amount;
        balances[_to] += _amount;
    }
}
```


**12. ERC-20 TOKENS:**

**Change string private salt = "value" to any yours.**
```solidity
// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

contract WeightedVoting is ERC20 {
    string private salt = "value"; 
    using EnumerableSet for EnumerableSet.AddressSet;

    error TokensClaimed();
    error AllTokensClaimed();
    error NoTokensHeld();
    error QuorumTooHigh();
    error AlreadyVoted();
    error VotingClosed();

    struct Issue {
        EnumerableSet.AddressSet voters;
        string issueDesc;
        uint256 quorum;
        uint256 totalVotes;
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 votesAbstain;
        bool passed;
        bool closed;
    }
    struct SerializedIssue {
        address[] voters;
        string issueDesc;
        uint256 quorum;
        uint256 totalVotes;
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 votesAbstain;
        bool passed;
        bool closed;
    }

    enum Vote {
        AGAINST,
        FOR,
        ABSTAIN
    }
    Issue[] internal issues;
    mapping(address => bool) public tokensClaimed;
    uint256 public maxSupply = 1000000;
    uint256 public claimAmount = 100;
    string saltt = "any";

    constructor(string memory _name, string memory _symbol)
        ERC20(_name, _symbol)
    {
        issues.push();
    }

  
    function claim() public {
        if (totalSupply() + claimAmount > maxSupply) {
            revert AllTokensClaimed();
        }
        if (tokensClaimed[msg.sender]) {
            revert TokensClaimed();
        }
        _mint(msg.sender, claimAmount);
        tokensClaimed[msg.sender] = true;
    }


    function createIssue(string calldata _issueDesc, uint256 _quorum)
        external
        returns (uint256)
    {
        if (balanceOf(msg.sender) == 0) {
            revert NoTokensHeld();
        }
        if (_quorum > totalSupply()) {
            revert QuorumTooHigh();
        }
        Issue storage _issue = issues.push();
        _issue.issueDesc = _issueDesc;
        _issue.quorum = _quorum;
        return issues.length - 1;
    }

   
    function getIssue(uint256 _issueId)
        external
        view
        returns (SerializedIssue memory)
    {
        Issue storage _issue = issues[_issueId];
        return
            SerializedIssue({
                voters: _issue.voters.values(),
                issueDesc: _issue.issueDesc,
                quorum: _issue.quorum,
                totalVotes: _issue.totalVotes,
                votesFor: _issue.votesFor,
                votesAgainst: _issue.votesAgainst,
                votesAbstain: _issue.votesAbstain,
                passed: _issue.passed,
                closed: _issue.closed
            });
    }

   
    function vote(uint256 _issueId, Vote _vote) public {
        Issue storage _issue = issues[_issueId];

        if (_issue.closed) {
            revert VotingClosed();
        }
        if (_issue.voters.contains(msg.sender)) {
            revert AlreadyVoted();
        }

        uint256 nTokens = balanceOf(msg.sender);
        if (nTokens == 0) {
            revert NoTokensHeld();
        }

        if (_vote == Vote.AGAINST) {
            _issue.votesAgainst += nTokens;
        } else if (_vote == Vote.FOR) {
            _issue.votesFor += nTokens;
        } else {
            _issue.votesAbstain += nTokens;
        }

        _issue.voters.add(msg.sender);
        _issue.totalVotes += nTokens;

        if (_issue.totalVotes >= _issue.quorum) {
            _issue.closed = true;
            if (_issue.votesFor > _issue.votesAgainst) {
                _issue.passed = true;
            }
        }
    }
}
```

**13. ERC-721 TOKENS:**

**Change string salt = "value" to any yours.** 
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC721/ERC721.sol";

interface ISubmission {
    struct Haiku {
        address author;
        string line1;
        string line2;
        string line3;
    }

    function mintHaiku(
        string memory _line1,
        string memory _line2,
        string memory _line3
    ) external;

    // function ownerOf(uint256 _id) external view returns (address);

    function counter() external view returns (uint256);

    function shareHaiku(uint256 _id, address _to) external;

    function getMySharedHaikus() external view returns (Haiku[] memory);
}

contract HaikuNFT is ERC721, ISubmission {
    Haiku[] public haikus;
    mapping(address => mapping(uint256 => bool)) public sharedHaikus;
    uint256 public haikuCounter;

    constructor() ERC721("HaikuNFT", "HAIKU") {
        haikuCounter = 1;
    }

    string salt = "value";

    function counter() external view override returns (uint256) {
        return haikuCounter;
    }

    function mintHaiku(
        string memory _line1,
        string memory _line2,
        string memory _line3
    ) external override {
        // Check if the haiku is unique
        string[3] memory haikusStrings = [_line1, _line2, _line3];
        for (uint256 li = 0; li < haikusStrings.length; li++) {
            string memory newLine = haikusStrings[li];
            // string memory newHaikuString = string(
            //     abi.encodePacked(haikusStrings[li])
            // );
            for (uint256 i = 0; i < haikus.length; i++) {
                Haiku memory existingHaiku = haikus[i];
                string[3] memory existingHaikuStrings = [
                    existingHaiku.line1,
                    existingHaiku.line2,
                    existingHaiku.line3
                ];

                for (uint256 eHsi = 0; eHsi < 3; eHsi++) {
                    string memory existingHaikuString = existingHaikuStrings[
                        eHsi
                    ];
                    if (
                        keccak256(abi.encodePacked(existingHaikuString)) ==
                        keccak256(abi.encodePacked(newLine))
                    ) {
                        revert HaikuNotUnique();
                    }
                }
            }
        }

        // Mint the haiku NFT
        _safeMint(msg.sender, haikuCounter);
        haikus.push(Haiku(msg.sender, _line1, _line2, _line3));
        haikuCounter++;
    }

    function shareHaiku(uint256 _id, address _to) external override {
        require(_id > 0 && _id <= haikuCounter, "Invalid haiku ID");

        Haiku memory haikuToShare = haikus[_id - 1];
        require(haikuToShare.author == msg.sender, "NotYourHaiku");

        sharedHaikus[_to][_id] = true;
    }

    function getMySharedHaikus()
        external
        view
        override
        returns (Haiku[] memory)
    {
        uint256 sharedHaikuCount;
        for (uint256 i = 0; i < haikus.length; i++) {
            if (sharedHaikus[msg.sender][i + 1]) {
                sharedHaikuCount++;
            }
        }

        Haiku[] memory result = new Haiku[](sharedHaikuCount);
        uint256 currentIndex;
        for (uint256 i = 0; i < haikus.length; i++) {
            if (sharedHaikus[msg.sender][i + 1]) {
                result[currentIndex] = haikus[i];
                currentIndex++;
            }
        }

        if (sharedHaikuCount == 0) {
            revert NoHaikusShared();
        }

        return result;
    }

    error HaikuNotUnique();
    error NotYourHaiku();
    error NoHaikusShared();
}
```
