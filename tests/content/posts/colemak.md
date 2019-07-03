```
template = "post"
title = "The Colemak keyboard layout"
date = "30th of June 2019"
reddit = "https://www.reddit.com/r/MechanicalKeyboards/comments/c7c4ea/the_colemak_keyboard_layout_or_one_weird_trick_to/"
hacker_news = "https://news.ycombinator.com/item?id=20317701"
```

In [my last post][] I mentioned that I don't use the ubiquitous <span class="sc">qwerty</span> keyboard layout, but instead an alternative layout called Colemak. The top voted comment was a request for a similarly toned article about Colemak, so here's a brief introduction to what Colemak is, and what my experience with it has been like.

First, some very brief history. <span class="sc">qwerty</span> became standard on typewriters in the late 19th century. People claim variously that the letter positions were chosen to avoid the levers jamming when keys close together were pressed in quick succession; to allow the word *typewriter* to be keyed in using the top row alone; or even to slow down fast typists to keep them from hitting the physical limits of the machine. Regardless, <span class="sc">qwerty</span> doesn't seem to have been subject to much in the way of ergonomic considerations, and a number of alternatives have sprung up since to address this deficiency. Dvorak and Colemak are the two most common alternative layouts, but before we get to specifics let's take a step back.

If we were designing a keyboard layout today -- without the historical baggage of the typewriter -- what might we want to optimise for? The easiest metric to reason about is finger travel. When we want to hit a key: how far do we have to stretch to get to it? Not every key is equally easy or comfortable to reach. We could come up with a scoring system for keys based on how much effort it is to hit them. Something like:

    8 7 6 5 9 9 5 6 7 8
    3 2 1 0 4 4 0 1 2 3
    8 7 6 5 9 9 5 6 7 8

where lower numbers indicate lower effort. Then we could arrange the most commonly used letters on the keys with the lowest scores. You could disagree on the scoring based on how difficult you think it is to hit the keys in the middle column, whether you find it easier to hit the top row than the bottom, if you have iron pinkies... but this simple model already captures a lot of the spirit of what we're trying to achieve. For an example of how this kind of optimisation can play out, here's a version of Genesis written using *only* the home row keys of the Colemak layout (from the [Colemak site][]):

> In the start The One has risen the stars and the earth.\
> The earth had no order, and nothin' resided there; and shade resided on the nonendin' 'neath. And The One rided on the seas.\
> Then The One said: "I desire it to shine"; and it shone.\
> And The One had seen the shine, that it's neat; and The One sorted the shine on one side, and the shade on the other.\
> The One then denoted the shine and the shade. So the nite and the shine that are date no. one had ended.

The distance from the home row is not the whole story. We also need to consider the transitions from one key to another. Certain sequences of letters occur more frequently than others, and certain transitions are more awkward than others. Dvorak's approach to this problem is to optimise for *hand alternation*: hitting keys with the left hand and the right hand alternately -- mostly by placing vowels on one half of the keyboard and the most commonly used consonants on the other. Colemak instead optimises for *same-hand rolls*: a fluid motion pressing multiple keys in turn with a single movement of the hand. From here on I'm going to talk about my experiences with Colemak, but I expect that a lot of this would apply just as readily to any optimised layout.

The summer of my first year of university I discovered the bottomless money pit that is the world of Mechanical Keyboards. After investing a small portion of my student loan in the very clicky and very plasticy [Cherry G80-3000][] I decided I should probably learn to touch type properly. Many [GNU Typist][] lessons and online typing tests later I was happily tapping away with impeccable 10 finger touch typing technique at 70 odd words per minute. I was content with <span class="sc">qwerty</span>, and it *felt* fine (I didn't know any better!) but I did notice that occasionally the stars would align and a certain word be especially pleasant to type. Wouldn't it be nice if the stars aligned more often?

Jump to the summer of my third year, and enter Colemak. I don't remember where I first read about it and how I decided that it might be worth trying, but I do remember choosing it over the alternative alternatives because:

1. it optimises for same-hand rolls (which was what I had identified as the source of the "stars aligning" feeling)
2. it shares many keys with <span class="sc">qwerty</span>, including common shortcut keys
3. it [performs marginally better than Dvorak][] on metrics which made sense to me
4. it has out of the box support in macOS and Linux (Dvorak does too, but many more obscure layouts don't)

Being a student, and it being the summer holidays, I had the opportunity to switch cold turkey. Back to GNU Typist I went.

During the first few days, every letter I typed involved consciously stopping my muscle memory from taking over, actively remembering where the key is, and only then hitting it. Frustrating is an understatement. Gradually though I got faster and faster at consciously finding the keys I wanted, and then started to let my muscle memory take the reins again. From that point on the process of feeling my brain re-wire itself became really rewarding. Here's a graph I found of my progress over the first 45 days:

![Colemak progress](/images/colemak-progress.png)

The sharp decline in gradient at day 14 corresponds to starting a software engineering internship, which also served as very good motivation to get to a reasonable speed in the first two weeks. I've long since stopped updating this graph, but still do the occasional typing test, and these days fairly commonly reach 100 <span class="sc">wpm</span>.

More important than raw speed, is that Colemak is very *comfortable* to use. That stars-aligning feeling that I'd noticed on <span class="sc">qwerty</span> seems to happen for almost every word on Colemak. Make the raw process of typing more enjoyable and everything you do which involves typing becomes that bit more rewarding too.

As far as convenience goes: I decided I would completely abandon <span class="sc">qwerty</span> from the start, and I've never been any the worse for it. Software support for Colemak is first class everywhere but Windows (but by the looks of it still pretty easy to set up there), and 99% of the time I use a [programmable keyboard][my last post] with my layout baked in to the firmware anyway.

The short but hard up front investment is not going to be worth it for a lot of people, but if you type for a living or are curious enough to read my unstructured ramblings all the way to the end... then Colemak might be for you. Otherwise, at the very least, typing this post was a pleasure.

[my last post]: 48-keys-are-plenty
[Colemak site]: https://colemak.com/Fun
[Cherry G80-3000]: https://www.cherry.co.uk/cherry-g80-3000.html
[GNU Typist]: https://www.gnu.org/savannah-checkouts/gnu/gtypist/gtypist.html
[performs marginally better than Dvorak]: http://mkweb.bcgsc.ca/carpalx/?colemak
