# emoji64
Welcome to emoji64! Have you ever wondered how you could make base64 even more human-incomprehensible, but at the same time making it look more cool?
You did not? Well I have the answer to this question anyway! I present to you emoji64, a tool that translates base64 output to emojis or vice-versa.

## How do I use this?
It's quite simple! emoji64 has two modes, encode and decode. It's default mode is encoding the input to Emojis. Simply pass the `--decode` (or `-d` for short) argument to it and it will switch to decoding the input. All input is read from stdin, so it plays nicely with pipes.

Here's an example of encoding and decoding the message "Hello World!":
```
echo "Hello World!" | base64 | emoji64 | emoji64 -d | base64 -d
```

The output in emoji64 would look like this:
```
😤😘😧🙄😳😘😉😸😧😃😊🙊😳😘😢😹😔😸😎😎
```

## Your Rust programming sucks.
Why, thank you! This is the first thing I've written after "Hello World" and it made some aspects of the language clearer to me. Normally I wouldn't torture you with my early abominations when learning a new language, but I joked with some colleagues that I would create this.

## So - What is the practical application?
There is none.

## I want to create an implementation in another language. How do you convert to Emojis?
The magic number I'm using is `-47 +0x1F600` to encode to Emoji. The emoticons start at the 0x1F600 range in Unicode and subtracting 47 ensures that the first possible character in base64 is also the very first Emoji in Unicode. Just apply that formula to your character to encode it, or reverse it to decode it.

## License
emoji64 is a tool to decode and encode base64 into Unicode based emoticons.
Copyright (C) 2016  Oliver "JTweet" Springer

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
