# Regex Postfix to Infix Notation

This code converts a regex from infix to postfix notation, inserting a symbol that marks the concatenation of regex building blocks.

The reason for this treatment is the content of the article: https://swtch.com/%7Ersc/regexp/regexp1.html by Ross Cox which describes the Thompson's algorithm described by Thompson in his 1968 CACM paper. The article is saved inside the doc folder for reference.

This algorithm starts with a conversion from postfix to infix (which is why this repository exists). This function outputs a . character for concatenated objects. This implementation also has a concatenation symbol which is printed using the # character.

This code is inspired by ??? cannot find the repository any more.

## Inner Workings

The algorithm builds up a (binary) tree from the regex in infix form by extending the tree character by character as the regex is consumed. Once constructed, traversing the tree in a recursive fashion and outputting a node after each of it's children have been visited yields the infix form.

To see how the tree is constructed in steps, check the Infix_to_Postfix_treebased.txt text file in the doc folder. It contains examples.

Because it is hard to implement data structures in safe rust without expert knowledge about the language, this implementation implements a tree (without parent pointers) using an arena base approach. An arena is basically a preallocated vector of nodes. The nodes point to the left and right child, not by Boxes, Rc, Weak or similar but by simple integers (usize) used as indexes into the vector to index the nodes that are the left and right child.

That way it is rather simple to implement a binary tree and to modify it dynamically as the tree grows and needs to be reorganized. The tree needs reorganization for example, when a literal follows a literal and then these two literals are reparanted into a new concatenation parent node for example.

Additionally, when a brace '(' opens, the index of that node is inserted into a separate root-node vector. That vector is used as a stack. The top element points to the innermost unclosed brace. When a brace is closed, the stack is reverted by one element and the topmost element becomes the new root of the tree. This allows for easier insertion of regex objects into nested braces.


