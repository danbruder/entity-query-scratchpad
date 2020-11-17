Tinkering with the idea of describing data loading needs in the entity layer of a system.

When building stuff with the clean architecture, it is nice to stay in the entity layer as much as possible and reduce the outer layers as much as possible. The way I've been doing things is to define an interface and implement it with postgres queries for each entity in the system. I'm wondering about defining a single interface that can CRUD json like a KV store and building a application-specific query specification on top of that.

The alarm bells are going off saying don't re-implement a crappy version of SQL for your entity layer to keep separation of concerns. But! as I've been working with clean architecture, I've used less and less database features - especially joins and aggregations, and more often I've been loading a bag of things by key and constructing domain entities in the entity layer from the bag of data (including joining data when necessary).

Ideas to explore:

- Handling aggregations in the entity layer
  - materialized view where the infra layer loads a bunch of data, the entity layer runs aggregations, and the results are cached
  - Incremental calculation of aggregated data
- Defining "cachable" stuff in the entity layer
- an api to define a join of data: "load this range and that range, then give me both ranges to map over and return"
- an api to define filters and selects

There's probably a ton of prior art here that I should research before going any further.

These ideas are an amalgomation of some books I've read over the past year:

- the clean architecture
- database internals
- dynamodb book
- designing data-intensive applications
- seven databases in seven weeks
