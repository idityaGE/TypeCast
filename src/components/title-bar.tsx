import { getCurrentWindow } from '@tauri-apps/api/window';

const TitleBar = () => {
  const appWindow = getCurrentWindow();

  return (
    <div className="titlebar h-3 hover:h-8 bg-gray-400 text-gray-800 duration-200 transition-all ease-in-out flex justify-between group fixed top-0 left-0 w-full z-50">
      <div data-tauri-drag-region className='h-full w-full'></div>
      <div className="controls hidden group-hover:flex delay-200">
        <button id="titlebar-minimize" title="minimize" onClick={() => appWindow.minimize()}>
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
            <path fill="currentColor" d="M19 13H5v-2h14z"></path>
          </svg>
        </button>
        <button id="titlebar-maximize" title="maximize" onClick={() => appWindow.toggleMaximize()}>
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
            <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />
          </svg>
        </button>
        <button id="titlebar-close" title="close" onClick={() => appWindow.hide()}>
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
            <path
              fill="currentColor"
              d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z"
            />
          </svg>
        </button>
      </div>
    </div>
  )
}

export default TitleBar