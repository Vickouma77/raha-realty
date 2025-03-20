<h1 style="text-align: center">Raha Realty Architecture</h1>

## Table of Contents
- [Introduction](#introduction)
- [Architecture](#architecture)
- [Components](#components)
- [Technologies](#technologies)
- [Deployment](#deployment)
- [Development](#development)
- [Testing](#testing)
- [Monitoring](#monitoring)
- [Security](#security)

## Introduction
This document describes the architecture of the Raha Realty application.

## Architecture

### High Level Architecture Overview
1. **Bounded Contexts** => The application is divided into multiple bounded contexts, each responsible for a specific domain: 
    - **Property Listing Context**
    - **User Context**
    - **Payment Context**
    - **Notification Context**
    - **Booking Context**
    - **Search Context**
    - **Review Context**
    - **Rating Context**

    Each context becomes a microservice, with its own data store and logic, ensuring clear boundaries and reducing coupling.

2. Event-Driven Architecture => The application is built using an event-driven architecture, where each microservice communicates with others through events. This ensures loose coupling and scalability.

3. **API Gateway** => The API Gateway is the single entry point for all clients. It routes requests to the appropriate microservice based on the request path.

4. **Service Discovery** => The microservices register themselves with the service registry, which allows other services to discover and communicate with them.

5. **Load Balancing** => The API Gateway and microservices are deployed behind a load balancer to distribute incoming traffic evenly across multiple instances.

6. **Caching** => Caching is used to improve performance and reduce load on the database. Each microservice has its own cache to store frequently accessed data.

7. **Database Sharding** => The databases are sharded to distribute the data across multiple nodes, improving scalability and performance.

8. **Monitoring and Logging** => The application is monitored using tools like Prometheus and Grafana. Logs are collected centrally using ELK stack.

9. **Security** => The application follows security best practices, including encryption, authentication, and authorization.

## Components

### Property Listing Context
- **Property Service** => Manages property listings, including creation, updating, and deletion.
- **Image Service** => Stores and serves property images.
- **Search Service** => Provides search functionality for properties.

### User Context
- **User Service** => Manages user accounts, including registration, login, and profile management.
- **Authentication Service** => Handles user authentication and authorization.

### Payment Context
- **Payment Service** => Manages payments for property bookings.

### Notification Context
- **Notification Service** => Sends notifications to users, including email and SMS notifications.

### Booking Context
- **Booking Service** => Manages property bookings, including creation, updating, and cancellation.

### Review Context
- **Review Service** => Manages property reviews, including creation, updating, and deletion.

### Rating Context
- **Rating Service** => Manages property ratings, including creation and updating.

## Microservices with Rust and Actix-Web
Each bounded context runs as a microservice built with Rust and Actix-web. Rust is a systems programming language known for its performance and safety guarantees. Actix-web is a high-performance web framework for Rust.


## Low Level Architecture Overview
1. **Property Listing Context**
    - **Property Service**
        - **DDD LAYER** => Domain-Driven Design (DDD) architecture with the following layers:
            - **Entities** => Property, Address, Image, etc.
            - **Repositories** => Interfaces for interacting with the database.
            - **Services** => Business logic for managing properties.
            - **Value Objects** => Value objects like Price, Area, etc.
            - **Factories** => Factories for creating entities.
            - **Aggregates** => Aggregates like Property, which encapsulate related entities.
        - **API** => REST API for managing property listings.
        - **Database** => PostgreSQL database for storing property data.
        - **Cache** => Redis cache for caching property data.
    - **Image Service**
        - **API** => REST API for storing and serving property images.
        - **Storage** => S3 bucket for storing images.
    - **Search Service**
        - **API** => REST API for searching properties.
        - **Index** => Elasticsearch index for storing property data.

2. **User Context**
    - ***DDD LAYER*** => Domain-Driven Design (DDD) architecture with the following layers:
        - **Entities** => User, Address, etc.
        - **Repositories** => Interfaces for interacting with the database.
        - **Services** => Business logic for managing user accounts.
        - **Value Objects** => Value objects like Email, Phone, etc.
        - **Factories** => Factories for creating entities.
        - **Aggregates** => Aggregates like User, which encapsulate related entities.
    - **User Service**
        - **API** => REST API for managing user accounts.
        - **Database** => PostgreSQL database for storing user data.
    - **Authentication Service**
        - **API** => REST API for handling user authentication and authorization.

3. **Payment Context**
    - **DDD LAYER** => Domain-Driven Design (DDD) architecture with the following layers:
        - **Entities** => Payment, Order, etc.
        - **Repositories** => Interfaces for interacting with the database.
        - **Services** => Business logic for managing payments.
        - **Value Objects** => Value objects like Amount, Currency, etc.
        - **Factories** => Factories for creating entities.
        - **Aggregates** => Aggregates like Order, which encapsulate related entities.
    - **Payment Service**
        - **API** => REST API for managing payments.
        - **Payment Gateway** => Integration with a payment gateway for processing payments.

4. **Notification Context**
    - **DDD LAYER** => Domain-Driven Design (DDD) architecture with the following layers:
        - **Entities** => Notification, Message, etc.
        - **Repositories** => Interfaces for interacting with the database.
        - **Services** => Business logic for sending notifications.
        - **Value Objects** => Value objects like Recipient, Content, etc.
        - **Factories** => Factories for creating entities.
        - **Aggregates** => Aggregates like Notification, which encapsulate related entities.
    - **Notification Service**
        - **API** => REST API for sending notifications.
        - **Email Service** => Integration with an email service for sending emails.
        - **SMS Service** => Integration with an SMS service for sending SMS notifications.

5. **Booking Context**
    - **DDD LAYER** => Domain-Driven Design (DDD) architecture with the following layers:
        - **Entities** => Booking, Property, User, etc.
        - **Repositories** => Interfaces for interacting with the database.
        - **Services** => Business logic for managing bookings.
        - **Value Objects** => Value objects like Check-in Date, Check-out Date, etc.
        - **Factories** => Factories for creating entities.
        - **Aggregates** => Aggregates like Booking, which encapsulate related entities.
    - **Booking Service**
        - **API** => REST API for managing property bookings.
        - **Database** => PostgreSQL database for storing booking data.

6. **Review Context**
    - **DDD LAYER** => Domain-Driven Design (DDD) architecture with the following layers:
        - **Entities** => Review, Property, User, etc.
        - **Repositories** => Interfaces for interacting with the database.
        - **Services** => Business logic for managing reviews.
        - **Value Objects** => Value objects like Rating, Comment, etc.
        - **Factories** => Factories for creating entities.
        - **Aggregates** => Aggregates like Review, which encapsulate related entities.
    - **Review Service**
        - **API** => REST API for managing property reviews.
        - **Database** => PostgreSQL database for storing review data.

7. **Rating Context**
    - **DDD LAYER** => Domain-Driven Design (DDD) architecture with the following layers:
        - **Entities** => Rating, Property, User, etc.
        - **Repositories** => Interfaces for interacting with the database.
        - **Services** => Business logic for managing ratings.
        - **Value Objects** => Value objects like Rating Value, etc.
        - **Factories** => Factories for creating entities.
        - **Aggregates** => Aggregates like Rating, which encapsulate related entities.
    - **Rating Service**
        - **API** => REST API for managing property ratings.
        - **Database** => PostgreSQL database for storing rating data.

## Technologies
- **Rust** => Systems programming language known for its performance and safety.
- **Actix-web** => High-performance web framework for Rust.
- **PostgreSQL** => Relational database for storing application data.
- **Redis** => In-memory data store for caching.
- **Elasticsearch** => Distributed search and analytics engine.
- **S3** => Object storage service for storing images.
- **Payment Gateway** => Integration with a payment gateway for processing payments.
- **Email Service** => Integration with an email service for sending emails.
- **SMS Service** => Integration with an SMS service for sending SMS notifications.
- **Docker** => Containerization platform for packaging applications.
- **Kubernetes** => Container orchestration platform for managing containerized applications.
- **Prometheus** => Monitoring and alerting toolkit.
- **Grafana** => Open-source analytics and monitoring platform.
- **ELK Stack** => Elasticsearch, Logstash, and Kibana for log management.
