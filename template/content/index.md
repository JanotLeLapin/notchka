---
title: Hello
js: [katex]
css:
  - base
  - code
katex: true # Include the KaTeX library and stylesheets
prism: true # Include the Prism library
---

# Welcome to Notchka

This is a template for the Notchka static site generator. Feel free to edit this file and see what happens!

## Mathematics

Notchka supports mathematical expressions out-of-the-box thanks to the Katex.js library. As an example, here's the quadratic formula:

$$\frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$

Write all the $\LaTeX$ you want between dollar signs (`$`) to render your maths!

## Code Snippets

Notchka also supports code snippets out-of-the-box through the Prism.js syntax highlighter. Here's some Python code:

```py
lst = ['Notchka', 'is', 'pretty', 'cool', '!']

for elem in lst:
    print(elem, end=' ')
```

Keep in mind, Notchka does not provide a Prism.js theme out of the box. You may find a theme you like [here](https://github.com/PrismJS/prism-themes), or make your own theme from scratch. Right now, you're seeing [**Material Light** by DutchenkoOleq](https://github.com/PrismJS/prism-themes/blob/master/themes/prism-material-light.css).

Try to guess what language this is:

```java
package main;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hi mom");
    }
}
```
