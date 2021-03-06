# Design

## Entities

### Message

Dictionary: string -> variant type. Might have some metadata, like a timestamp.

Basic implementation may be a string.

### Node

Node encapsulates a function and possibly some state.

In the most basic implementation, node is a function pointer.

In a more advanced implementation, node is a function plus the same dictionary as used in the Message.

Specific types of nodes may have some user-configurable properties (instance variables).

We can also distinguish a separate entity, "Node Class", which is a template for creating nodes of the given type. The
function is a property of the node class. Other node class properties are its visual representation, help text,
properties list, etc.

### Flow

Flow is a graph of nodes. It describes how nodes are connected.

The minimal implementation might be a graphviz format file.

In a more advanced implementation, nodes might have appearance, comments, placement on the canvas, etc.

### Flow execution

Apparently nodes aren't always simply functions. Nodes may also produce messages asynchronously.

Chain of events:

```dot
A -> B
B -> C
B -> D
```

Event queue.

Asynchronous events can add to the event queue. An out-event is (message, producer_node). out-event can be replaced by
in-events for every consumer node:
(message, consumer_node).

consumer_node function is invoked with the message as an argument. If it produces something, the result is added as an
out-event.

This avoids deep recursion.

Nodes need to generate events asynchronously. But the node doesn't know about connections, Flow does. When node is
created, it needs to be given a function/closure to call when it wants to generate asynchronous events. This will hook
into the flow executor, which will place out-event onto the event queue.

## Dispatch loop

- AsyncDispatcher::dispatch gets called with message as an argument. It places the message into the dispatch queue. This
  can happen from any task. Dispatcher must keep `Arc<Mutex<Queue<Message>>>` or similar.
- Well, not exactly a message because we also need to know where to deliver it. Source node is also not sufficient,
  since the node may have multiple outputs. Probably need to keep a tuple of Message, source node, output index.
- Shall we wrap source node + output index into "port"?..
- FlowState::run_once gets a message+source from the queue, finds the destination, calls the destination handler, puts
  the result (if any) into the queue.

# TODOs

- [ ] Add "manual" trigger nodes
- [ ] Figure out how and where to implement the dispatch loop.
