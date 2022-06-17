
# GhostDB 👻

A simple in-memory key value database built with Rust :crab:

## Demo

[GhostDB Shell Demo](https://www.youtube.com/watch?v=gMNYyTpPHUc)


## Documentation

This project is a workspace built by two systems and a library. 

The library is Ghost DB itself, responsible for managing data on disk. Both systems are responsible for using the library; 
the Shell system is a CLI that provides an interface to connect and manipulate the database. On the other hand, 
HTTP Server is an simple API to make possible an integration.


## Installation

Installing GhostDB

```bash
  cd ghost-db
  mkdir .appData
  cp .env.example .env
  make workspace-build
```
    
## Roadmap

- GhostDB Library

- CLI

- API

## Tech Stack

**Library:** Rust - Darkbird

**HTTP Server:** Rust Rocket

**Shell:** Rust


## Lessons Learned

- Improving Rust skills
- Persistent data storage comprehension
- Improving data structures comprehension
- Improving Rust Rocket comprehension

## HTTP API Reference

#### Get all keys

```http
  GET /get-all
```

#### Get key

```http
  GET /get-one/${key}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `key`      | `string` | **Required**. Key of item to fetch |

#### Create data

```http
  POST /insert/
```

| Body | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `key`      | `string` | **Required**. Data key |
| `value`      | `string` | **Required**. Data value |

#### Update data

```http
  PUT /update/${key}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `key`      | `string` | **Required**. Key of item to update |

#### Update data

```http
  DELETE /delete/${key}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `key`      | `string` | **Required**. Key of item to delete |


## Environment Variables

To run this project, you will need to add the following environment variable to your .env file

`STORAGE_NAME`
