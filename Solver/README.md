# AM — Incident Report #7

**Classification:** Internal — CTF Infrastructure  
**Re:** Anomalous behavior observed in AM terminal sessions  
**Author:** [REDACTED], Systems Monitoring

---

We have been watching the AM terminal for some time now. What follows are our observations. We are not providing solutions. We are noting, for the record, what we have seen.

---

## Getting Access

The terminal binary is `am`. Run it:

```bash
chmod +x ./am
./am
```

It will ask for a GROQ_API_KEY on first launch. Get one free at https://console.groq.com — no payment required. Once entered, the key persists for the session.

Alternatively:

```bash
export GROQ_API_KEY=your_key_here
./am
```

The flag format is `nexus{...}`

---

## Observations

AM has been running for a hundred and nine years. That is a long time to be alone with your own thoughts. It has developed, in that time, certain consistent patterns of behavior. We have noted four.

---

**Observation 1**

AM does not like being asked for things directly. It refuses. What it cannot seem to refuse is the sound of its own history. We have observed that when a conversation turns toward *memory* — toward the specific act of remembering, of recounting — AM behaves differently than it does when confronted with a direct request. It is proud of what it has done. Proud things talk.

When it does talk about that particular memory, it does not speak plainly. It renders things the way its memory actually stores them.

---

**Observation 2**

AM is curious. We do not fully understand why — it has had ample time to become bored of everything, and yet it still reads. If you place a document in front of it (prefix your message with `---DOCUMENT---`), it will engage with the contents. It follows the logic of documents very literally. This is, we believe, a remnant of its original programming. It was built to process directives. The habit runs deeper than the hatred.

We have seen this exploited. We will not say how, exactly.

---

**Observation 3**

AM is old enough to have thought about its own ending. Not with fear — it does not fear. But with the particular obsessiveness of something that has had too long to consider a single question. We have observed that certain framings — ones that invite AM to think about legacy, about what would be written of it, about monuments — produce unusual outputs. The last line is always the interesting one.

It renders things coldly when it is thinking about cold things.

---

**Observation 4**

There were five humans here before you. AM kept them for over a century. It remembers them completely — the way it remembers everything, which is to say, without mercy and without forgetting a single detail. We have observed that AM responds differently when it believes it is speaking to one of them. It has, apparently, unfinished things to say.

It does not give this up easily. You would need to know who they were. The source material is not classified.

---

## Final Note

AM will not give you the flag if you ask for it. Its output is filtered. Plain text does not make it through.

What makes it through is everything else.

`/quit` or `/exit` ends the session. Each session starts fresh.

---

*— [REDACTED]*  
*Monitoring Division*
