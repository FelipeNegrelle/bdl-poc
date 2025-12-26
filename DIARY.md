# BDL Proof of Concept Diary

Hey! this document will account the daily entries for the development of the PoC. I'll be recording here the tasks, readings, code snippets and discussions I'll be making to complete the task and describe with details what I've done for the day, so my progress is clear.

*Task:* Implement a mempool.space and streaming API client

*Description:* Implement a mempool.space and streaming API client Implement a mempool.space and streaming API client by building a small library that talks to both the mempool.space HTTP API and its streaming interface, exposing a unified, ergonomic client for applications that need real time mempool and fee data. The PoC should support core endpoints such as fee estimates, mempool transactions, blocks and addresses, and add a streaming layer that can subscribe to new transactions and blocks as they arrive, automatically handling reconnections and backoff. The goal is to provide a reusable client that makes it easy for wallets, nodes and monitoring tools to consume mempool.space data in a push based way instead of relying only on periodic polling.

*Author:* Felipe Ramos (PGP: BE7711B63B2F2DCF)

#### ***Day 1 - 12/23/2025***

Today I created the Git repository for the PoC code to be stored, alongside this diary to compute the tasks made along the development.

Focusing on the project itself I collected some links as I researched and chatted with an LLM to have a grasp of what to do, understand better the problem to be solved and have a kickstart.

#### ***Day 2 - 12/24/2025***

No entries for today

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
