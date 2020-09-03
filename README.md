# oc-genblog

overcodes tool to generate a static blog.

# Example

Write your blog post:

politics-are-overrated.md
```
<!-- @template "blog-post.html" -->
# Politics are overrated

Politics are overrated, but discussion is a necesary evil to ensure we fight the creep of authoritative government.
```

Generate the post:

```
oc-genblog politics-are-overrated.md > politics-are-overrated.html
```

# Templates

The key to making this more than just a Markdown to HTML convert is the use of templates that have plugins; for example, we can write an HTML template that uses
one of the default templates to populate the last-modified date.

blog-post.html.template
```
<html>
    <body>
        <h1>{{ attributes(key="title") }} <small>last-updated {{ last_modified() }}</small></h1>
        {{ content() }}
    </body>
</html>
```

The templates are powered by Tera, and you can use of the features provided by that template language.
We define some context objects which can be used:

- attribute(key="some key") -- used to fetch an attribute specified in the markdown file; for example, 'template' above
- content() -- fetches the HTML representation of the Markdown file
- last_modified() -- gets the last-modified timestamp from the file in question