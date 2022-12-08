<!-- @format -->

# Day 6: Tuning Trouble

## Part One

The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the <span style="color:#ffff66;text-shadow: 0 0 2px #ffff66;">star fruit</span> grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the most important one to set up right now is the <span style="color:#fff;text-shadow: 0 0 2px #fff;">communication system</span>.

However, because he's heard you have [significant](https://adventofcode.com/2016/day/6) [experience](https://adventofcode.com/2016/day/25) [dealing](https://adventofcode.com/2019/day/7) [with](https://adventofcode.com/2019/day/9) [signal-based](https://adventofcode.com/2019/day/16) [systems](https://adventofcode.com/2021/day/25), he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to <span style="color:#fff;text-shadow: 0 0 2px #fff;">lock on to their signal</span>. The signal is a series of seemingly-random characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a <span style="color:#fff;text-shadow: 0 0 2px #fff;">start-of-packet marker</span> in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of <span style="color:#fff;text-shadow: 0 0 2px #fff;">four characters that are all different</span>.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

`mjqjpqmgbljsphdztnvjfqwrcgsmlb`

After the first three characters (`mjq`) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters `mjqj`. Because `j` is repeated, this isn't a marker.

The first time a marker appears is after the <span style="color:#fff;text-shadow: 0 0 2px #fff;">seventh</span> character arrives. Once it does, the last four characters received are `jpqm`, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after <span style="color:#fff;text-shadow: 0 0 2px #fff;">`7`</span> characters have been processed.

Here are a few more examples:

- `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character<span style="color:#fff;text-shadow: 0 0 2px #fff;">`5`</span>
- `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character <span style="color:#fff;text-shadow: 0 0 2px #fff;">`6`</span>
- `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character <span style="color:#fff;text-shadow: 0 0 2px #fff;">`10`</span>
- `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character <span style="color:#fff;text-shadow: 0 0 2px #fff;">`11`</span>

<span style="color:#fff;text-shadow: 0 0 2px #fff;">How many characters need to be processed before the first start-of-packet marker is detected?</span>

To begin, [get your puzzle input](https://adventofcode.com/2022/day/6/input).

## Part Two

Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for <span style="color:#fff;text-shadow: 0 0 2px #fff;">messages</span>.

A <span style="color:#fff;text-shadow: 0 0 2px #fff;">start-of-message marker</span> is just like a start-of-packet marker, except it consists of <span style="color:#fff;text-shadow: 0 0 2px #fff;">14 distinct characters</span> rather than 4.

Here are the first positions of start-of-message markers for all of the above examples:

- `mjqjpqmgbljsphdztnvjfqwrcgsmlb`: first marker after character <span style="color:#fff;text-shadow: 0 0 2px #fff;">`19`</span>
- `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character <span style="color:#fff;text-shadow: 0 0 2px #fff;">`23`</span>
- `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character <span style="color:#fff;text-shadow: 0 0 2px #fff;">`23`</span>
- `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character <span style="color:#fff;text-shadow: 0 0 2px #fff;">`29`</span>
- `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character <span style="color:#fff;text-shadow: 0 0 2px #fff;">`26`</span>

<span style="color:#fff;text-shadow: 0 0 2px #fff;">How many characters need to be processed before the first start-of-message marker is detected?</span>
