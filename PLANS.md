# Elasticsearch Recommended Products
This project is used to learn more about Elasticsearch. I found a [post](https://spinscale.de/posts/2021-12-08-using-elasticsearch-transforms-for-product-recommendations.html) about using Elasticsearch as an engine to get recommended products for a product(s). Alexander is using tools as transform, ingest and pipelines to convert order items into orders and then use these orders to get recommendations. I will probably not use the same way to develop the same results as I want to develop this via an API. My goal is to in the future build some kind of an API around this system to use this for other systems.

# Plans
- How could I scale this system to be able to use it on multiple nodes in a big cluster. Am I able to scale this worldwide in some smart way?
- Be able to get results based on users information

# In what way do I plan to develop this.
There are many ways to accomplish this, My goal is to be able to use this system in a production environment which means we need to have this system automatic. I've seen way to acomplish this by uloading CSV files into Elastic which is not a way I want to do this.

# Resources references
- [High-Quality Recommendation Systems with Elasticsearch](https://opensourceconnections.com/blog/2016/09/09/better-recsys-elasticsearch/)
- [Building Simple Recommender Systems for Elasticsearch](https://qbox.io/blog/building-simple-recommender-systems-for-elasticsearch-1/)
- [Using Elasticsearch Transforms For Product Recommendations](https://spinscale.de/posts/2021-12-08-using-elasticsearch-transforms-for-product-recommendations.html)
