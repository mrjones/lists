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

var ListPicker = React.createClass({
  getInitialState: function() {
    return {lists: []};
  },
  componentDidMount: function() {
    $.ajax({
      url: `/lists/${this.props.params.userId}`,
      dataType: 'json',
      cache: false,
      success: function(data) {
        console.log(data);
        this.setState({lists: data});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(this.props.url, status, err.toString());        
      }.bind(this)
    });
  },
  render: function() {
    var listNodes = this.state.lists.map(function(list) {
      return (
        <div className="list" key={list.id}>
          {list.name}
        </div>
      );
    });
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
    <ReactRouter.Route path="/lists/:userId" component={ListPicker} />
  </ReactRouter.Router>
  ), document.getElementById('content'));
