import numpy as np
from numpy import genfromtxt
import matplotlib.pyplot as plt
from matplotlib import cm
from mpl_toolkits import mplot3d


data = genfromtxt('data.csv', delimiter=',')
data_s = data[:,1:].reshape(11,800,2)
plt.plot(np.sum(data_s[:,:,0], axis=0))
plt.show()

data_lines = data_s[:,:,0]


for a in data_lines:
    a -= a.min()
    a /= a.max()

plt.plot(data_lines[-4:].transpose())
plt.show()

for i in range(11):
    data_s[i,:,0] = data_s[i,:,1][np.argsort(np.argsort(data_s[i,:,0]))]

data_s[:,:,0] = data_s[:,:,0] - 600

viridis = cm.get_cmap('viridis', 11)
for (a, b) in zip(data_s, viridis(range(11))):
    plt.scatter(x=a[:,1], y=a[:,0], color=b, s=4)
plt.show()

plt.plot(np.sum(data_s[:,:,0], axis=0))
plt.show()


ax = plt.axes(projection='3d')
ax.set_proj_type('ortho')
for count, (a, b) in enumerate(zip(data_s, viridis(range(11)))):
    ax.scatter(a[:,1], a[:,0], np.full((800), count))
plt.show()
