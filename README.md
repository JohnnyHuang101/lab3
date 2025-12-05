# CSE 5402 Fall 2025 Lab 2

## Teammates
    * Aman Verma - aman.v@wustl.edu
    * Johnny Huang - h.johnny@wustl.edu
    * Hanson Li - hanson.l@wustl.edu

## Structure
``` 
├── data
│   ├── abro_repeat.txt
│   ├── Guildenstern_hamlet_ii_2a.txt
│   ├── hamlet_ii_1a_config.txt
│   ├── hamlet_ii_1b_config.txt
│   ├── hamlet_ii_2a_config.txt
│   ├── hbro_repeat.txt
│   ├── jbro_repeat.txt
│   ├── King_hamlet_ii_2a.txt
│   ├── narrator_repeat.txt
│   ├── Ophelia_hamlet_ii_1b.txt
│   ├── partial_hamlet_act_ii_script.txt
│   ├── Polonius_hamlet_ii_1a.txt
│   ├── Polonius_hamlet_ii_1b.txt
│   ├── Queen_hamlet_ii_2a.txt
│   ├── Reynaldo_hamlet_ii_1a.txt
│   ├── Rosencrantz_hamlet_ii_2a.txt
│   ├── test_1_AJbro_1a.txt
│   ├── test_1_hrbo_1a.txt
│   ├── test_1_narrator_1a.txt
│   ├── test_1_script.txt
│   ├── test_2_empty_config.txt
│   ├── test_2_script.txt
│   └── test_hamlet_ii_config.txt
├── README.md
├── README.txt
├── src
│   ├── lab2
│   │   ├── declarations.rs
│   │   ├── mod.rs
│   │   ├── player.rs
│   │   ├── play.rs
│   │   ├── return_wrapper.rs
│   │   ├── scene_fragments.rs
│   │   └── script_gen.rs
│   └── main.rs 
```

## Program Overview
* Our program consists of a Sever rust package **lab3server** responsible for serving the play files stored in the folder and a Client rust package **lab3cleint.rs** responsible for ingesting both local play files (in our 'data' folder) (also have a test client **lab3testclient** that tests server connection and remote file serving). 
    * **server: lab3server** w
     * **client: lab3client** w
* **local/remote file structure:**
    * our local files are stored at the data folder. To access script, config, or speak files from our client, other script/config files 
    * our remote files are stored inside the lab3server directory. To access script, config, or speak files from our client, other script/config files, we must specify the file location in this format on the command line or inside the caller script/config txt files: ```<ip_address>:<port_number>:<file_name>```. Example: net:127.0.0.1:8124:partial_macbeth_act_i_script.txt

## Insights/Observations
* ### Client/Server setup
    * Our client does a bulk of the compute, whereas the server is just mainly responsible for serving its remote files to the client. From doing this project and from the Piazza discussion, we learned about the fat client/thin server and fat server/thin client concepts. 
    * We noticed when implemeting the Run method for the Server, there wasn't a requirment for using buffered reader and writers. We wonder if this is because for our thin server approach, we are only reading in one line and printing out a handful of lines so a buffer is not needed. But in a fat server approach where the server handles the actual processing, a buffered reader/writer may be needed.
    * We noticed one advantage with the fat client setup is the ability to handle local/remote file mixes. With the addition of the get_buffered_reader function in script_gen, our fat client can process both local and remote server files for a play. However, we think it'd be more complex for a fat server to retrieve local client files--this would likely invovle the client sending the file to the server, which makes a fat server/thin client setup more diffcult for this project.
* ### Deadlocks 
    * One challenge we spent alot of time on debugging was deadlocks with overuse of .lock(). It was hard to pinpoint where the deadlock was happening, so initally we removed all locks and iteratively added back in locks only at the necessary locations. 

# Usage
* CD into our unzipped folder, it should contain the data, lab3server, lab3client, and lab3testclient folders.
* **Step 1: To start server**: cd into lab3server, ```cargo run <ip_addr>:<port_num>``` for our project we used ```cargo run 127.0.0.1:8124```. This is the ip/port combo for all our remote files in our script and config txt.
* **Optional: To test server connection:** cd into lab3testclient, ```cargo run <ip_addr>:<port_num> <name of file inside lab3server folder>```. Running this, server should respond with printed contents of the file.
* **Step 2: To start and run client**: ```cargo run <path to script file>```
    * Starting with a local script file:```<path to script file>``` is relative path to the local file
    * Starting with a remote script file: ```<path to script file>``` is ```<ip_addr>:<port_num>:<file_name>```. Example: ```cargo run net:127.0.0.1:8124:partial_hamlet_act_ii_script.txt``` (if partial_hamlet_act_ii_script.txt is inside lab3server folder)


# Testing
* we tested the provided partial_hamelt-act_ii_script.txt and partial_macbeth_act_i_script.txt, verified their output and line orders, and the outputs are stored at
* Local, Remote, and Local/Remote mix testing:
    * for lab provided play script/config/speak files, we store local copies of it in the **data** folder and the remote versions within the **lab3server** folder.
    * testing local and remote scripts separatley:
        * to test fully local scripts for the play, we ran the ```cargo run ../data/partial_hamlet_act_ii_script.txt``` and ```cargo run net:127.0.0.1:8124:partial_macbeth_act_i_script.txt```. The lab-provied script and config files all reference speak files in the local data folder with the '../data/. prefix. Testing showed program outputting the correct in-order delviery for both parts
        * to test fully remote scripts for the play, we ran the ```cargo run net:127.0.0.1:8124:partial_hamlet_act_ii_script.txt``` and the ```cargo run net:127.0.0.1:8124:partial_macbeth_act_i_script.txt```. All script and config files in the **lab3server** folders reference speak files stored in the folder with the **net:127.0.0.1:8124:** prefix. Testing also showed correct delivery for both plays.
    * testing mixed local and remote scripts:
        * to test lab-provied play scripts with local/remote mixing: 
            * we created the lab3server/localremotemix_partial_hamlet_act_ii_script.txt files that references a fully local hamlet_ii_1a_config.txt file, a fully remote hamlet_ii_1b_config.txt file, and a local/remote mixed localremotemix_hamlet_ii_2a_config.txt file (this file references both local and remote speak files). Testing our program shows correct delivery for the hamlet act 2 script. 
            * We also created the lab3server/localremotemix_partial_macbeth_act_i_script.txt file, that references a fully local macbeth_i_1_config.txt and macbeth_i_2b_config.txt file and fully remote macbeth_i_2a_config.txt file. Testing shows correct delievery for the macbeth act 1 script.
        * additional local/remote mixed tests:
            * we created a johnny_file.txt with corresponding config file johnny_file_config.txt in **lab3server** folder. In our local **data** folder, we added the johnny_file_config.txt into the test_1_script.txt file. We then ran this local script file with ```cargo run ../data/test_1_script.txt```, and our program correctly printed out boht the local speak files in **data** and the added remote file johnny_file.txt. 
* Stress testing with multiple clients:
    * 
* Out of order lines testing:
    * we moved lines 6 and 9 of the FIRST_WITCH_macbeth_i_1.txt to the top of the file, and re-ran both the local mode with all part files in the data folder and the remote mode with the remote files served from the lab3server folder. Both tests showed the correct order of First Witch's lines, showing that our sort_by still works after the refactoring for this lab
    * we have more out of order lines in part files associated with the test_1_script.txt which will be discussed below. These out of order lines also appear in correct order from our tests

* **test_1_script.txt** is our test script from lab2 that we refactored for this lab to add another scene that include a config/speech file from our server. its updated to contain 4 config txt files (test_1_hrbo_1a.txt, test_1_AJbro_1a.txt, test_1_narrator_1a.txt, johnny_file_config.txt) split into 3 scenes with 4 local speak files hrbo_repeat.txt, jbro_repeat.txt, narrator_repeat.txt, abro_repeat.txt in the 'local' data folder and a remote speak file johnny_file.txt in the server folder that is served by our server. 
    * new test cases added for this lab:
        * having a remote config file in local script file:
            * the test_1_script.txt is in our local 'data' folder. However, it references net:127.0.0.1:8124:johnny_file_config.txt, which is a remote file that lives in our server folder. Testing shows our program called the server twice (for the johnny_file_configt.txt and johnny_file.txt that lives on the server), and correctly printed out all lines among local and remote config/speak files. 
        * large gaps in line number:
            * in johnny_file.txt, the line numbers begins with a large offset (begins at 100). Program output shows that even with this large offset, the program delivers all line in the correct sequence 
    * old test cases that we continued to test and passed:
        * repeated line numbers where the line content is different. This repeated line number also carries across files (ex 2 abro_repeat and jbro_repeat.txt shares line 7, and hbro_repeat.txt and narrator_repeat.txt shares line 0)
            * our program handles by delivering them in sequence. It whinges if whinge mode is on, for every dupelicated line
        * out of order line numbers
            * during testing we discovered our program didn't correctly deliver these out of order line numbers for a speak file in order because we dind't sort the PlayLines field in Player struct in the prepare method. We added that back in and the issue was resolved
        * lines with number only
            * when parsing lines with only line number and no text content, our program correctly ignores that line and does not print it out
        * lines with no numbers
            * when parsing lines missing a line number, our program correcty ignores that line and if Whinge is on, it gives a warning about missing line number
* **test_2_script.txt** is another testing script with two scenes (from lab2). The first scene doesn't have a config file under it, and the second scene has an empty config file 'test_2_empty_config.txt'.
    * we tested this script file again with no changes. Just like our program in lab 2, our new program exited read_config with error code 2' and main function will return the GENERATION_FAILURE code. This confirms our refactoring handles issues in the local mode the same as our lab2 program. 

# Testing outputs:
* from the provided partial_hamlet_act_ii_script.txt: ./partial_hamlet_output.txt
* from the provided partial_macbeth_act_i_script.txt: ./partial_macbeth_output.txt
* from our 1st test file test_1_script.txt: ./test_1_output.txt
* from our 2nd test file test_2_script.txt: ./test_2_output.txt
