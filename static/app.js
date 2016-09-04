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
        <h1>ListsApp!</h1>
        {React.cloneElement(this.props.children, {
           userId: this.props.params.userId
         })}
      </div>
    );
  }
});

var ListItem = React.createClass({
  render: function() {
    var linkNodes = this.props.data.link_annotations.map(function(link) {
      return (
        <a href={link.url} key={link.url}>{link.url}</a>
      )
    });

    return (
      <li className="listItem">
        <div className="name">{this.props.data.name}</div>
        <div className="description">{this.props.data.description}</div>
        {linkNodes}
      </li>
    );
  }
});

var AddItemWidget = React.createClass({
  getIntialState: function() {
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
      name: this.state.name.trim(),
      description: this.state.description.trim(),
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
        console.log("posted some data");
      }.bind(this),
      error: function(xhr, status, err) {
        this.setState({data: comments});
        console.error(this.props.url, status, err.toString());
      }.bind(this)
    });

    this.setState({name: '', descrption: ''});
  },
  render: function() {
    return (
      <div>
        Add Item:
        <form onSubmit={this.handleSubmit}>
          <input name="name" placeholder="Name" type="text" onChange={this.handleNameChange} />
          <br/>
          <textarea name="description" placeholder="Description" onChange={this.handleDescriptionChange} />
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
  componentDidMount: function() {
    $.ajax({
      url: `/lists/${this.props.params.userId}/list/${this.props.params.listId}`,
      dataType: 'json',
      cache: false,
      success: function(data) {
        console.log(data);
        this.setState({name: data.name, items: data.items});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error("url", status, err.toString());        
      }.bind(this)
    });
  },
  render: function() {
    var itemNodes = this.state.items.map(function(item) {
      return (
        <ListItem data={item} key={item.id}/>
      );
    });
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
        <AddItemWidget userId={this.props.params.userId} listId={this.props.params.listId}/>
      </div>
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
  render: function() {
    var listNodes = this.state.lists.map(function(list) {
      return (
        <div className="list" key={list.id}>
          <ReactRouter.Link to={`/lists/${this.props.userId}/list/${list.id}`}>
            {list.name}
          </ReactRouter.Link>
        </div>
      );
    }, this);
    return (
      <div>
        {listNodes}
      </div>
    );
  }
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
