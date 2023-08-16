use spin::Once;
use syscall_table::{register_syscall, Table};
static SYSCALL_TABLE: Once<Table> = Once::new();
pub fn register_all_syscall(){
	let mut table = Table::new();
	register_syscall!(table,
	(160, uname),
	(215, do_munmap),
	(222, do_mmap),
	(226, map_protect),
	(227, msync),
	(233, madvise),
	(283, membarrier),
	(169, get_time_of_day),
	(153, times),
	(101, nanosleep),
	(113, clock_get_time),
	(102, getitimer),
	(103, setitimer),
	(114, clock_getres),
	(115, clock_nanosleep),
	(93, do_exit),
	(94, exit_group),
	(124, do_suspend),
	(154, set_pgid),
	(155, get_pgid),
	(157, set_sid),
	(172, get_pid),
	(173, get_ppid),
	(174, getuid),
	(175, geteuid),
	(176, getgid),
	(177, getegid),
	(178, get_tid),
	(220, clone),
	(221, do_exec),
	(260, wait4),
	(214, do_brk),
	(96, set_tid_address),
	(261, prlimit64),
	(116, syslog),
	(179, sys_info),
	(118, sched_setparam),
	(121, sched_getparam),
	(122, sched_setaffinity),
	(123, sched_getaffinity),
	(120, sched_getscheduler),
	(119, sched_setscheduler),
	(165, getrusage),
	(40, sys_mount),
	(39, sys_umount),
	(56, sys_openat),
	(57, sys_close),
	(61, sys_getdents),
	(45, sys_truncate),
	(46, sys_ftruncate),
	(63, sys_read),
	(64, sys_write),
	(17, sys_getcwd),
	(49, sys_chdir),
	(83, sys_mkdir),
	(62, sys_lseek),
	(80, sys_fstat),
	(37, sys_linkat),
	(35, sys_unlinkat),
	(36, sys_symlinkat),
	(78, sys_readlinkat),
	(79, sys_fstateat),
	(44, sys_fstatfs),
	(43, sys_statfs),
	(276, sys_renameat),
	(34, sys_mkdirat),
	(5, sys_setxattr),
	(6, sys_lsetxattr),
	(7, sys_fsetxattr),
	(8, sys_getxattr),
	(9, sys_lgetxattr),
	(10, sys_fgetxattr),
	(11, sys_listxattr),
	(12, sys_llistxattr),
	(13, sys_flistxattr),
	(14, sys_removexattr),
	(15, sys_lremovexattr),
	(66, sys_writev),
	(65, sys_readv),
	(67, sys_pread),
	(68, sys_pwrite),
	(16, sys_fremovexattr),
	(71, send_file),
	(81, sync),
	(82, fsync),
	(25, fcntl),
	(29, ioctl),
	(88, utimensat),
	(48, faccessat),
	(52, chmod),
	(53, chmodat),
	(166, unmask),
	(73, ppoll),
	(72, pselect6),
	(134, sigaction),
	(137, sigtimewait),
	(135, sigprocmask),
	(129, kill),
	(130, tkill),
	(139, signal_return),
	(133, sigsuspend),
	(194, shmget),
	(196, shmat),
	(195, shmctl),
	(59, sys_pipe),
	(23, sys_dup),
	(24, sys_dup2),
	(98, futex),
	(99, set_robust_list),
	(100, get_robust_list),
	(2003, system_shutdown),
	(198, socket),
	(200, bind),
	(201, listening),
	(202, accept),
	(203, connect),
	(204, getsockname),
	(205, get_peer_name),
	(206, sendto),
	(207, recvfrom),
	(208, setsockopt),
	(209, getsockopt),
	(210, shutdown),
	(199, socket_pair),
	(2002, sys_event_get),
	(2000, sys_framebuffer),
	(2001, sys_framebuffer_flush),

	);
	SYSCALL_TABLE.call_once(||table);
}
pub fn do_syscall(id:usize,args:&[usize])->Option<isize>{
	let res = SYSCALL_TABLE.get().unwrap().do_call(id,&args);
	res
}
use crate::device::sys_event_get;
use crate::fs::chmod;
use crate::fs::chmodat;
use crate::fs::faccessat;
use crate::fs::fcntl;
use crate::fs::fsync;
use crate::fs::ioctl;
use crate::fs::ppoll;
use crate::fs::pselect6;
use crate::fs::send_file;
use crate::fs::sync;
use crate::fs::sys_chdir;
use crate::fs::sys_close;
use crate::fs::sys_fgetxattr;
use crate::fs::sys_flistxattr;
use crate::fs::sys_fremovexattr;
use crate::fs::sys_fsetxattr;
use crate::fs::sys_fstat;
use crate::fs::sys_fstateat;
use crate::fs::sys_fstatfs;
use crate::fs::sys_ftruncate;
use crate::fs::sys_getcwd;
use crate::fs::sys_getdents;
use crate::fs::sys_getxattr;
use crate::fs::sys_lgetxattr;
use crate::fs::sys_linkat;
use crate::fs::sys_listxattr;
use crate::fs::sys_llistxattr;
use crate::fs::sys_lremovexattr;
use crate::fs::sys_lseek;
use crate::fs::sys_lsetxattr;
use crate::fs::sys_mkdir;
use crate::fs::sys_mkdirat;
use crate::fs::sys_mount;
use crate::fs::sys_openat;
use crate::fs::sys_pread;
use crate::fs::sys_pwrite;
use crate::fs::sys_read;
use crate::fs::sys_readlinkat;
use crate::fs::sys_readv;
use crate::fs::sys_removexattr;
use crate::fs::sys_renameat;
use crate::fs::sys_setxattr;
use crate::fs::sys_statfs;
use crate::fs::sys_symlinkat;
use crate::fs::sys_truncate;
use crate::fs::sys_umount;
use crate::fs::sys_unlinkat;
use crate::fs::sys_write;
use crate::fs::sys_writev;
use crate::fs::unmask;
use crate::fs::utimensat;
use crate::gui::sys_framebuffer;
use crate::gui::sys_framebuffer_flush;
use crate::ipc::futex;
use crate::ipc::get_robust_list;
use crate::ipc::kill;
use crate::ipc::set_robust_list;
use crate::ipc::shmat;
use crate::ipc::shmctl;
use crate::ipc::shmget;
use crate::ipc::sigaction;
use crate::ipc::signal_return;
use crate::ipc::sigprocmask;
use crate::ipc::sigsuspend;
use crate::ipc::sigtimewait;
use crate::ipc::sys_dup;
use crate::ipc::sys_dup2;
use crate::ipc::sys_pipe;
use crate::ipc::tkill;
use crate::memory::do_mmap;
use crate::memory::do_munmap;
use crate::memory::madvise;
use crate::memory::map_protect;
use crate::memory::membarrier;
use crate::memory::msync;
use crate::net::accept;
use crate::net::bind;
use crate::net::connect;
use crate::net::get_peer_name;
use crate::net::getsockname;
use crate::net::getsockopt;
use crate::net::listening;
use crate::net::recvfrom;
use crate::net::sendto;
use crate::net::setsockopt;
use crate::net::shutdown;
use crate::net::socket;
use crate::net::socket_pair;
use crate::sbi::system_shutdown;
use crate::sys::getrusage;
use crate::sys::sched_getaffinity;
use crate::sys::sched_getparam;
use crate::sys::sched_getscheduler;
use crate::sys::sched_setaffinity;
use crate::sys::sched_setparam;
use crate::sys::sched_setscheduler;
use crate::sys::sys_info;
use crate::sys::syslog;
use crate::system::uname;
use crate::task::clone;
use crate::task::do_brk;
use crate::task::do_exec;
use crate::task::do_exit;
use crate::task::do_suspend;
use crate::task::exit_group;
use crate::task::get_pgid;
use crate::task::get_pid;
use crate::task::get_ppid;
use crate::task::get_tid;
use crate::task::getegid;
use crate::task::geteuid;
use crate::task::getgid;
use crate::task::getuid;
use crate::task::prlimit64;
use crate::task::set_pgid;
use crate::task::set_sid;
use crate::task::set_tid_address;
use crate::task::wait4;
use crate::timer::clock_get_time;
use crate::timer::clock_getres;
use crate::timer::clock_nanosleep;
use crate::timer::get_time_of_day;
use crate::timer::getitimer;
use crate::timer::nanosleep;
use crate::timer::setitimer;
use crate::timer::times;
