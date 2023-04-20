INSERT INTO log ( log, error, priority, send, extra, subdomain, appname) values (
  'first log', false, 3, false, 'extra message', 'abc', 'demo'
);

INSERT INTO log ( log, error, priority, send, extra, subdomain, appname) values (
  'second log', false, 3, true, 'extra message', 'abc', 'demo'
);

INSERT INTO log ( log, subdomain, appname) values (
  'third log', 'abc', 'demo'
)
;
INSERT INTO log ( log, error, priority, send, extra, subdomain, appname) values (
  'forth log', false, 3, false, 'extra message', 'abc', 'demo'
);

INSERT INTO log ( log, error, priority, send, extra, subdomain, appname) values (
  'fifth log', false, 3, true, 'extra message', 'abc', 'demo'
);

INSERT INTO log ( log, subdomain, appname) values (
  'sixth log', 'abc', 'demo'
);
