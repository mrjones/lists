var UserPicker = React.createClass({
  getInitialState : function() {
    return {users: [
      {name: "Matt", id: 1},
      {name: "Cristina", id:2},
    ]};
  },
  render: function() {
    var userNodes = this.state.users.map(function(user) {
      return (
        <div className="user" key={user.id}>
          {user.name}
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

var Widget = React.createClass({
  render: function() {
    return (
      <UserPicker />
    );
  }
});

ReactDOM.render(
  <Widget />,
  document.getElementById('content')
);

