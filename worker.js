let handle = null;
const interval = 100;

onmessage = (e) => {
  switch (e.data) {
    case 'start':
      console.log("starting");
      handle = setInterval(() => {
        postMessage("tick");
      }, interval);
      break;
    case 'stop':
      console.log("stopping");
      clearInterval(handle);
      handle = null;
      break;
  }
};

postMessage('worker loaded');
