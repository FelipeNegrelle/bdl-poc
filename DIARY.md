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