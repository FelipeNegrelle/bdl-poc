# BDL Proof of Concept Diary

Hey! this document will account the daily entries for the development of the PoC. I'll be recording here the tasks, readings, code snippets and discussions I'll be making to complete the task and describe with details what I've done for the day, so my progress is clear.

*Task:* Implement a mempool.space and streaming API client

*Description:* Implement a mempool.space and streaming API client by building a small library that talks to both the mempool.space HTTP API and its streaming interface, exposing a unified, ergonomic client for applications that need real time mempool and fee data. The PoC should support core endpoints such as fee estimates, mempool transactions, blocks and addresses, and add a streaming layer that can subscribe to new transactions and blocks as they arrive, automatically handling reconnections and backoff. The goal is to provide a reusable client that makes it easy for wallets, nodes and monitoring tools to consume mempool.space data in a push based way instead of relying only on periodic polling.

*Author:* Felipe Ramos (PGP: BE7711B63B2F2DCF)

#### ***Day 1 - 12/23/2025***

Today I created the Git repository for the PoC code to be stored, alongside this diary to compute the tasks made along the development.

Focusing on the project itself I collected some links as I researched and chatted with an LLM to have a grasp of what to do, understand better the problem to be solved and have a kickstart.

#### ***Day 2 - 12/24/2025***

No entries for today.

#### ***Day 3 - 12/25/2025***

For today I started exploring the endpoints to the client to integrate. I've found the mempool.space docs for their API's and found the following:

#### REST APIs endpoints (https://mempool.space/api):
- fee estimates: https://mempool.space/api/v1/fees/precise
- mempool transactions: https://mempool.space/api/mempool/txids
- blocks:
    - https://mempool.space/api/blocks/:start_block_height (get n blocks)
    - https://mempool.space/api/block/:blockhash
- addresses:
    - https://mempool.space/api/address/:address (get address basic info)
    - https://mempool.space/api/address/:address/txs (transactions from an adress)

#### Websocket endpoints (wss://mempool.space/api/v1/ws):
- subscribe to new transactions on demand -> *"track-txs"*
- subscribe to blocks on demand -> *"track-mempool-block"*

Reading their docs I've discovered that the mempool.space API is public and don't need auth to work and that they have rate limiting and throthling, that if exploited several times you get banned from using their API. I've found that they have a paid plan for extended API usage and that it's quite expensive. I explored the docs and cherry-picked the endpoints that meet the requirements in the description of the PoC and structured this topics here to further reference and as a base for the implementation. Finally, I created the rust project with cargo and added basic dependencies for HTTP requests and Websocket connections.

Merry Christmas!

#### ***Day 4 - 12/26/2025***

No entries for today.

#### ***Day 5 - 12/27/2025***

No entries for today.

#### ***Day 6 - 12/28/2025***

Today I started my journey through learning what I need to build my rust integration with mempool.space API. So, at first glance I went to chat with Claude AI to see if I could just get the pieces of knowledge necessary to build my application but my experience was terrible and it only confused me more than helped me understand the concepts of rust programming. 

After this I went old-school mode and started reading the rust docs from their website from the beggining and read and tested interactively with Cargo, TOML files and very basic rust source codes to understand the language paradigm and how it works for real. I stopped on the second chapter, called: "*2. Programming a Guessing Game.*"

#### ***Day 7 - 12/29/2025***

No entries for today.

#### ***Day 8 - 12/30/2025***

No entries for today.

#### ***Day 9 - 12/31/2025***

Following the rust learning journey and continued exploring the docs from the site, finished the guessing game section, and I think that learning like that: reading the docs and experimenting with the CLI and code works much better than with video crash courses or through AI LLMs, at least personally, cause I felt that I could actually comprehend stuff.

Happy New Year!

#### ***Day 10 - 01/01/2026***

For the first of the year I studied the basics of variables and its mutability in rust, from the rust docs. What I found interesting was the redeclaration of variables that works a little bit different to what I was used to, with it being able to redeclarate with another types and reuse that in different ways.

#### ***Day 11 - 01/02/2026***

Today was a big day. Firstly, to confirm the details and doubts I had about the PoC I talked with leonardolima on discord, who oriented me on improving the making of my project and confirmed the idea of building a lib to integrate with mempool space in rust which could be integrated with BDK.

After that I returned to the rust language studies, going from topic 3 to 5 and learned a lot. I understood about the difference about arrays and tuples, the special *unit* type and how to specify types in rust. I learnt about functions, the conceptual differences about parameters and arguments, also statements and expressions. After that, I explored loops, and the very curious feature of labeled loops. Then, came the most important concept of this day which was ownership, a very interesting matter to free ourselves from manually managing memory and potentially commiting mistakes. Rust makes all this easier with ownership, borrowing and references that haves it's own mutability. There's also the difference of types of where they allocate in memory and how they interact with the code lifecycle itself.

#### ***Day 12 - 01/03/2026***

Today still in my rust studies I've read through structs, and how to organize personalized data in a very OOP way with structs and it's methods, and also associated functions. This was very important to learn, because this topic will be specially useful to build the request and response models for the API endpoints implementations.

#### ***Day 13 - 01/04/2026***

No entries for today.

#### ***Day 14 - 01/05/2026***

Today I explored the enum section of the docs, focusing in understanding how to create personalized types that don't need a struct, but represent fixed values and also how to use the *match* operator to make kind of a *switch-case* statement to filter values in this control flow structure.

#### ***Day 15 - 01/06/2026***

Today I studied about the project directories and file organization in rust, learning about crates, that can be binary or libs, modules, submodules, the paths, the *use* and *mod* keywords, how the visibility of modules works and how to import stuff properly in rust code. This is a solid base to how to build my project better.

#### ***Day 16 - 01/07/2026***

No entries for today.

#### ***Day 17 - 01/08/2026***

No entries for today.

#### ***Day 18 - 01/09/2026***

Today I joined leonardolima office hours and he helped to organize better my plan of action, because I was stuck in what exactely to do in the project, what to prioritize and how could i build stuff in rust, since it is a language i don't have much experience. He suggested to just put everything in a file to start off, and fix this later and to prioritize the streaming client that connects to the websocket of mempool.space. So, this is what I am going to do!

#### ***Day 19 - 01/10/2026***

Today I started experimenting with the rust project, and understanding how exactely I'm going to do stuff, i chose the *tokio* and *tokio-tungstenite* libraries to communicate with the API and the *serde* library to serialize/deserialize the responses from the JSON, I will also use the *backoff* library to manage automatic reconnection with the API. With the dependencies chosen, I tried to understand how should I use them. The docs for the libraries are really confusing and I tried finding related material to use as reference, and chatted with ChatGPT about the possible structure, methods and logic of the project. I made some tests in the repo to understand how the API works.
