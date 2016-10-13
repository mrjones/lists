var sockets_api = require('./sockets_api_pb.js');

var UserPicker = React.createClass({
  getInitialState : function() {
    return {users: []};
  },
  componentDidMount: function() {
    $.ajax({
      url: '/users',
      dataType: 'json',
      cache: false,
      success: function(data) {
        console.log(data);
        this.setState({users: data});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(this.props.url, status, err.toString());        
      }.bind(this)
    });
  },
  render: function() {
    var userNodes = this.state.users.map(function(user) {
      return (
        <div className="user" key={user.id}>
          <ReactRouter.Link to={`/lists/${user.id}`}>
            {user.name}
          </ReactRouter.Link>
        </div>
      );
    });
    
    return (
      <div className="userPicker">
        {userNodes}
      </div>
    );
  },
});

var App = React.createClass({
  render: function() {
    return (
      <div>
        <h1>Welcome to the Lists App!</h1>
        {React.cloneElement(this.props.children, {
           userId: this.props.params.userId
         })}
      </div>
    );
  }
});

var ListItem = React.createClass({
  getInitialState: function() {
    return {
      addingLinkAnnotation: false,
      pendingLinkAnnotation: '',
      addingTextAnnotations: false,
      pendingTextAnnotation: ''
    }
    return this.stateForItem(this.props.data);
  },
  delete: function() {
    this.props.deleteFn(this.props.data.id);
  },
  postAnnotation: function(annotationObj) {
    var url = `/lists/${this.props.userId}/list/${this.props.listId}/items/${this.props.data.id}/annotations`;

    $.ajax({
      url: url,
      dataType: 'json',
      type: 'POST',
      data: JSON.stringify(annotationObj),
      success: function(data) {
        console.log("posted new annotation. got respnse: " + JSON.stringify(data));
        this.props.itemUpdatedFn(data);
//        this.setState(this.stateForItem(data));
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(url, status, err.toString());
      }.bind(this),
    });
  },
  toggleTextAnnotationAdder: function() {
    this.setState({addingTextAnnotation: !this.state.addingTextAnnotation});
  },
  pendingTextAnnotationChanged: function(e) {
    this.setState({pendingTextAnnotation: e.target.value});
  },
  addTextAnnotation: function() {
    // TODO: The "TEXT" string should live on the server
    this.postAnnotation({kind: "TEXT", body: this.state.pendingTextAnnotation});
    this.setState({addingLinkAnnotation: false, pendingLinkAnnotation: ''});
  },
  toggleLinkAnnotationAdder: function() {
    this.setState({addingLinkAnnotation: !this.state.addingLinkAnnotation});
  },
  pendingLinkAnnotationChanged: function(e) {
    this.setState({pendingLinkAnnotation: e.target.value});
  },
  addLinkAnnotation: function() {
    // TODO: The "link" string should live on the server
    this.postAnnotation({kind: "LINK", body: this.state.pendingLinkAnnotation});
    this.setState({addingLinkAnnotation: false, pendingLinkAnnotation: ''});
  },
  render: function() {
    var linkNodes = this.props.data.link_annotations.map(function(link) {
      return (
        <div key={link.url}>
          <a href={link.url}>{link.url}</a>
        </div>
      )
    });

    var streetEasyNodes = this.props.data.streeteasy_annotations.map(function(listing) {
      return (
        <div key={listing.hash}>
          Price: ${listing.price_usd}
        </div>
      );
    });

    var textNodes = this.props.data.text_annotations.map(function(text) {
      return (
        <div key={text.id}>
          {text.text}
        </div>
      );
    });

    var addLinkNodes;
    if (this.state.addingLinkAnnotation) {
      addLinkNodes =
        <div>
          <input type="text" placeholder="Url..." value={this.state.pendingLinkAnnotation} onChange={this.pendingLinkAnnotationChanged}/>
          <button onClick={this.addLinkAnnotation}>+</button>
        </div>;
    }

    var addTextNodes;
    if (this.state.addingTextAnnotation) {
      addTextNodes =
        <div>
          <input type="text" placeholder="Text..." value={this.state.pendingTextAnnotation} onChange={this.pendingTextAnnotationChanged}/>
          <button onClick={this.addTextAnnotation}>+</button>
        </div>;
    }

    return (
      <li className="listItem">
        <div>
          <span className="name">{this.props.data.name}</span>
          <button onClick={this.delete}>X</button>
          <button onClick={this.toggleLinkAnnotationAdder}>
            {this.state.addingLinkAnnotation ? "-URL" : "+URL"}
          </button>
          <button onClick={this.toggleTextAnnotationAdder}>
            {this.state.addingTextAnnotation ? "-Text" : "+Text"}
          </button>
        </div>
        <div className="description">{this.props.data.description}</div>
        {linkNodes}
        {streetEasyNodes}
        {textNodes}
        {addLinkNodes}
        {addTextNodes}
      </li>
    );
  }
});

var AddItemWidget = React.createClass({
  getInitialState: function() {
    return {name: '', description: ''};
  },
  handleNameChange: function(e) {
    this.setState({name: e.target.value});
  },
  handleDescriptionChange: function(e) {
    this.setState({description: e.target.value});
  },
  handleSubmit: function(e) {
    e.preventDefault();
    var item = {
      name: this.state.name,
      description: this.state.description,
    };
    if (!item.name || !item.description) {
      return;
    }

    $.ajax({
      url: `/lists/${this.props.userId}/list/${this.props.listId}/items`,
      dataType: 'json',
      type: 'POST',
      data: JSON.stringify(item),
      success: function(data) {
        console.log("posted new item. got respnse: " + JSON.stringify(data));
        this.props.itemAddedFn(data);
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(this.props.url, status, err.toString());
      }.bind(this)
    });

    this.replaceState(this.getInitialState(), function() {
      this.forceUpdate();
    }.bind(this));
  },
  render: function() {
    return (
      <div>
        Add Item:
        <form onSubmit={this.handleSubmit}>
          <input name="name" placeholder="Name" type="text" value={this.state.name} onChange={this.handleNameChange} />
          <br/>
          <textarea name="description" placeholder="Description" value={this.state.description} onChange={this.handleDescriptionChange} />
          <br/>
          <input type="submit" />
        </form>
      </div>
    );
  }
});

var List = React.createClass({
  getInitialState: function() {
    return {name: "", items: []}
  },
  itemAdded: function(item) {
    console.log("List::itemAdded: " + JSON.stringify(item));
    this.setState({items: this.state.items.concat([item])});
  },
  deleteItem: function(id) {
    $.ajax({
      url: `/lists/${this.props.params.userId}/list/${this.props.params.listId}/items/${id}`,
      type: 'DELETE',
      dataType: 'json',
      cache: false,
      success: function(data) {
        this.itemDeleted(id);
      }.bind(this),
      error: function(xhr, status, err) {
        console.error("url", status, err.toString());        
      }.bind(this)
    });
  },
  itemDeleted: function(id) {
    console.log("Removing item with id: " + id);
    this.setState({items: this.state.items.filter(function(item) {
      return item.id != id;
    })});
  },
  itemUpdateReceived: function(e) {
    console.log("Got " + e.data + " from WS server");
    this.handleItemUpdate(JSON.parse(e.data));
  },
  handleItemUpdate: function(json_data) {
    var new_item = json_data
    var new_items = this.state.items.map(function(old_item) {
      console.log("COMPARIING: " + JSON.stringify(old_item) + " vs. " + JSON.stringify(new_item));
      if (old_item.id == new_item.id) {
        console.log("new");
        return new_item;
      } else {
        console.log("old");
        return old_item;
      }
    });

    console.log("new items: " + JSON.stringify(new_items));
    this.setState({items: new_items}, function() {
      console.log("Mutation applied");
      console.log(JSON.stringify(this.state));
    });
  },
  componentDidMount: function() {
    $.ajax({
      url: `/lists/${this.props.params.userId}/list/${this.props.params.listId}`,
      dataType: 'json',
      cache: false,
      success: function(data) {
        this.setState({name: data.name, items: data.items});

        // TODO(mrjones): Negotiate the host / port?
        var endpoint = "ws://" + window.location.hostname + ":2347";
        var update_conn = new WebSocket(endpoint);
        update_conn.onopen = function() {
          var request = new sockets_api.Request();
          request.setWatchListId(this.props.params.listId);
//          var request = new sockets_api.Request();
//          request.setWatchListRequest(watchListRequest);
          console.log("Serialized: " + request.serializeBinary());
          update_conn.send(request.serializeBinary());
//          update_conn.send("watch:" + this.props.params.listId);
        }.bind(this);
        update_conn.onmessage = this.itemUpdateReceived;
      }.bind(this),
      error: function(xhr, status, err) {
        console.error("url", status, err.toString());        
      }.bind(this)
    });
  },
  render: function() {
    var itemNodes = this.state.items.map(function(item) {
      return (
        <ListItem data={item} key={item.id} deleteFn={this.deleteItem} userId={this.props.params.userId} listId={this.props.params.listId} itemUpdatedFn={this.handleItemUpdate}/>
      );
    }.bind(this));
    return (
      <div>
        <div>
          ListID: {this.props.params.listId} / {this.state.name}
        </div>
        <div>
          <ul>
            {itemNodes}
          </ul>
        </div>
        <AddItemWidget userId={this.props.params.userId} listId={this.props.params.listId} itemAddedFn={this.itemAdded} />
        <SharingWidget myUserId={this.props.params.userId} listId={this.props.params.listId} />
      </div>
    );
  }
});

var NewListWidget = React.createClass({
  getInitialState: function() {
    return {listName: ""};
  },
  handleSubmit: function(e) {
    e.preventDefault();
    var url = `/lists/${this.props.userId}/list`;
    var list = {name: this.state.listName};
    $.ajax({
      url: url,
      dataType: 'json',
      type: 'POST',
      data: JSON.stringify(list),
      success: function(data) {
        this.props.listAddedFn(data);
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(url, status, err.toString());
      }.bind(this),
    });
  },
  handleListNameChange(e) {
    this.setState({listName: e.target.value});
  },
  render: function() {
    return (
      <form onSubmit={this.handleSubmit}>
        <input name="name" type="text" placeholder="New list name" value={this.state.listName} onChange={this.handleListNameChange} />
        <input type="submit" value="+" />
      </form>
    );
  }
});

var ListPicker = React.createClass({
  getInitialState: function() {
    return {lists: []};
  },
  componentDidMount: function() {
    $.ajax({
      url: `/lists/${this.props.userId}`,
      dataType: 'json',
      cache: false,
      success: function(data) {
        console.log(data);
        this.setState({lists: data});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error("/lists", status, err.toString());        
      }.bind(this)
    });
  },
  removeList: function(e) {
    var list_id = e.target.value;
    var url = `/lists/${this.props.userId}/list/${list_id}`;
    console.log("Deleting " + list_id);
    $.ajax({
      url: url,
      type: 'DELETE',
      dataType: 'json',
      cache: false,
      success: function(data) {
        this.listRemoved(list_id);
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(url, status, err.toString());        
      }.bind(this)
    });
  },
  listRemoved: function(list_id) {
    this.setState({lists: this.state.lists.filter(function(list) {
      return list.id != list_id;
    })});
  },
  listAdded: function(list) {
    this.setState({lists: this.state.lists.concat([list])});
  },
  render: function() {
    var listNodes = this.state.lists.map(function(list) {
      return (
        <li className="list" key={list.id}>
          <ReactRouter.Link to={`/lists/${this.props.userId}/list/${list.id}`}>
            {list.name}
          </ReactRouter.Link>
          &nbsp;<button onClick={this.removeList} value={list.id}>X</button>
        </li>
      );
    }, this);
    return (
      <div>
        <ul>
          {listNodes}
        </ul>
        <NewListWidget userId={this.props.userId} listAddedFn={this.listAdded}/>
      </div>
    );
  }
});

var SharingWidget = React.createClass({
  getInitialState: function() {
    return {
      sharedWithLoaded: false,
      allUsersLoaded: false,
      sharedWithUsers: [],
      allUsers: []
    };
  },
  byId: function(a, b) {
    return a.id - b.id;
  },
  fetchAccessors: function() {
    var url = `/lists/${this.props.myUserId}/list/${this.props.listId}/accessors`;
    $.ajax({
      url: url,
      dataType: 'json',
      cache: false,
      success: function(data) {
        data.sort(this.byId);
        this.setState({sharedWithLoaded: true, sharedWithUsers: data});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(url, status, err.toString());        
      }.bind(this)
    });
  },
  fetchAllUsers: function() {
    var url = `/users`
    $.ajax({
      url: url,
      dataType: 'json',
      cache: false,
      success: function(data) {
        data.sort(this.byId);
        this.setState({allUsersLoaded: true, allUsers: data});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(url, status, err.toString());
      }.bind(this),
    });
  },
  componentDidMount: function() {
    this.fetchAccessors();
    this.fetchAllUsers();
  },
  assertSortedById: function(a) {
    for (var i = 1; i < a.length; i++) {
      if (a[i-1].id > a[i].id) {
        console.error("Not sorted at index " + i);
      }
    }
  },
  unsharedUsers: function() {
    this.assertSortedById(this.state.allUsers);
    this.assertSortedById(this.state.sharedWithUsers);

    var unshared = [];
    var sharedIdx = 0;
    for (var allIdx = 0; allIdx < this.state.allUsers.length; allIdx++) {
      while (sharedIdx < this.state.sharedWithUsers.length &&
             (this.state.sharedWithUsers[sharedIdx].id <
               this.state.allUsers[allIdx].id)) {
        sharedIdx++;
      }
      if (sharedIdx == this.state.sharedWithUsers.length ||
          (this.state.sharedWithUsers[sharedIdx].id >
            this.state.allUsers[allIdx].id)) {
        unshared.push(this.state.allUsers[allIdx]);
      }
    }

    return unshared;
  },
  addUserToList: function(userId) {
    var url = `/lists/${this.props.myUserId}/list/${this.props.listId}/accessors`
    $.ajax({
      url: url,
      dataType: 'json',
      type: 'POST',
      data: JSON.stringify({id: userId}),
      cache: false,
      success: function(data) {
        this.setState({sharedWithUsers: data});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(url, status, err.toString());
      }.bind(this),
    });
  },
  removeUserFromList: function(userId) {
    var url = `/lists/${this.props.myUserId}/list/${this.props.listId}/accessors`
    $.ajax({
      url: url,
      dataType: 'json',
      type: 'DELETE',
      data: JSON.stringify({id: userId}),
      cache: false,
      success: function(data) {
        this.setState({sharedWithUsers: data});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(url, status, err.toString());
      }.bind(this),
    });
  },
  render: function() {
    if (!this.state.sharedWithLoaded || !this.state.allUsersLoaded) {
      return (
        <div>Loading...</div>
      );
    }

    var allUserNodes = this.state.sharedWithUsers.map(function(user) {
      var deleteButton;
      if (this.props.myUserId != user.id) {
        deleteButton = (
          <button onClick={this.removeUserFromList.bind(this, user.id)}>X</button>
        );
      }
      return <li key={user.id}>{user.name}{deleteButton}</li>;
    }.bind(this));

    var unsharedUserNodes = this.unsharedUsers().map(function(user) {
      return <li key={user.id}>
          {user.name}
          <button onClick={this.addUserToList.bind(this, user.id)}>+</button>
      </li>;
    }.bind(this));
    
    return (
      <div>
        Shared with:
        <ul>
          {allUserNodes}
        </ul>

        Not Shared with:
        <ul>
          {unsharedUserNodes}
        </ul>
      </div>
    );
  },
});

// ReactDOM.render(
//  <Widget />,
//  document.getElementById('content')
// );

// https://www.kirupa.com/react/creating_single_page_app_react_using_react_router.htm
ReactDOM.render((
  <ReactRouter.Router>
    <ReactRouter.Route path="/" component={UserPicker} />
    <ReactRouter.Route path="/lists/:userId" component={App}>
      <ReactRouter.IndexRoute component={ListPicker} />
      <ReactRouter.Route path="list/:listId" component={List} />
    </ReactRouter.Route>
  </ReactRouter.Router>
  ), document.getElementById('content'));
